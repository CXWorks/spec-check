pub open spec fn powercap_measurements_notify__3_10_3_16_spec(agent_id: UInt32, domain_id: UInt32, notify_enable: UInt32, power_thresh_low: UInt32, power_thresh_high: UInt32, status: ScmiStatusCode, old_s: S, new_s: S) -> bool {
    (!IsValidPowercapDomain(old_s, domain_id) ==> (status == NOT_FOUND && new_s == old_s))
    && ((IsValidPowercapDomain(old_s, domain_id)
        && ((notify_enable & 0xFFFF_FFFEu32) != 0u32
            || !PowercapMeasurementsThresholdsAreLegal(old_s, domain_id, power_thresh_low, power_thresh_high)))
        ==> (status == INVALID_PARAMETERS && new_s == old_s))
    && ((status != SUCCESS) ==> new_s == old_s)
    && ((IsValidPowercapDomain(old_s, domain_id)
        && (notify_enable & 0xFFFF_FFFEu32) == 0u32
        && PowercapMeasurementsThresholdsAreLegal(old_s, domain_id, power_thresh_low, power_thresh_high))
        ==> (status == SUCCESS
            && PowercapMeasurementsNotifyEnabled(new_s, agent_id, domain_id) == ((notify_enable & 1u32) == 1u32)
            && PowercapPowerThreshLow(new_s, agent_id, domain_id) == power_thresh_low
            && PowercapPowerThreshHigh(new_s, agent_id, domain_id) == power_thresh_high
            && PowercapMeasurementsNotifyConfigUnchangedExcept(old_s, new_s, agent_id, domain_id)))
}
