pub open spec fn powercap_measurements_notify__3_10_3_16_spec(
    result: int32,
    old_s: S,
    new_s: S,
    domain_id: uint32,
    notify_enable: uint32,
    power_thresh_low: uint32,
    power_thresh_high: uint32,
) -> bool {
    // Failure conditions
    // NOT_FOUND: if domain_id does not point to a valid domain
    (domain_id == NOT_FOUND_DOMAIN_ID ==> ResultEqual(result, RMI_ERROR_NOT_FOUND))
    // INVALID_PARAMETERS: if any of the parameters specify values that are either illegal or incorrect
    // Reserved bits (Bits[31:1] of notify_enable) must be zero
    ((notify_enable & 0x7FFFFFFF) != 0 ==> ResultEqual(result, RMI_ERROR_INVALID_PARAMETERS))
    // Success condition
    // SUCCESS: if all parameters are valid
    (ResultEqual(result, RMI_SUCCESS) ==> (
        // domain_id must be valid
        IsDomainValid(old_s, domain_id)
        // notify_enable bit 0 can be 0 or 1
        (notify_enable & 1) == 0 || (notify_enable & 1) == 1
        // Reserved bits must be zero
        (notify_enable & 0x7FFFFFFF) == 0
        // power_thresh_low and power_thresh_high must be valid (non-negative, within range)
        power_thresh_low >= 0 && power_thresh_low <= MAX_POWER_THRESHOLD
        power_thresh_high >= 0 && power_thresh_high <= MAX_POWER_THRESHOLD
        // If notify_enable is 0, thresholds are ignored (no constraint on their values relative to each other)
        // If notify_enable is 1, thresholds must be valid (already checked above)
        // State transitions: thresholds are updated in new_s if notify_enable is 1
        (notify_enable & 1) == 1 ==> (
            RealmAt(new_s, domain_id).power_thresh_low == power_thresh_low
            && RealmAt(new_s, domain_id).power_thresh_high == power_thresh_high
        )
        // If notify_enable is 0, thresholds are not updated (remain as in old_s)
        (notify_enable & 1) == 0 ==> (
            RealmAt(new_s, domain_id).power_thresh_low == RealmAt(old_s, domain_id).power_thresh_low
            && RealmAt(new_s, domain_id).power_thresh_high == RealmAt(old_s, domain_id).power_thresh_high
        )
    ))
}