pub open spec fn powercap_measurements_notify__3_10_3_16_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidPowercapDomain(old_s, domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!AreValidParameters(old_s, domain_id(old_s), notify_enable(old_s), power_thresh_low(old_s), power_thresh_high(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> (MeasurementsNotifyEnabled(new_s, domain_id(old_s)) == (notify_enable(old_s) & 1) == 1))
    && (ResultEqual(result, SUCCESS) ==> PowerThreshLow(new_s, domain_id(old_s)) == power_thresh_low(old_s))
    && (ResultEqual(result, SUCCESS) ==> PowerThreshHigh(new_s, domain_id(old_s)) == power_thresh_high(old_s))
}