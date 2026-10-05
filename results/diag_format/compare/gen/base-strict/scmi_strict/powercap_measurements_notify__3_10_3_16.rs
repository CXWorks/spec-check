pub open spec fn powercap_measurements_notify__3_10_3_16_spec(status: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
    && (!AreValidMeasurementsNotifyParameters(old_s, domain_id, notify_enable, power_thresh_low, power_thresh_high) ==> ResultEqual(status, INVALID_PARAMETERS))
    && (ResultEqual(status, SUCCESS) ==> (MeasurementsNotifyEnabled(old_s, CallingAgent(), domain_id) == Bits(notify_enable, 0, 0))
        && (PowerThresholdLow(old_s, CallingAgent(), domain_id) == power_thresh_low)
        && (PowerThresholdHigh(old_s, CallingAgent(), domain_id) == power_thresh_high))
}