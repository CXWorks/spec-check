pub open spec fn powercap_measurements_notify__3_10_3_16_spec(domain_id: UInt32, notify_enable: UInt32, power_thresh_low: UInt32, power_thresh_high: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!AreValidParameters(old_s, domain_id, notify_enable, power_thresh_low, power_thresh_high) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> MeasurementsNotifyEnabled(new_s, caller, domain_id) == (notify_enable[0] == 1))
  && (ResultEqual(status, SUCCESS) ==> PowerThreshLow(new_s, caller, domain_id) == power_thresh_low)
  && (ResultEqual(status, SUCCESS) ==> PowerThreshHigh(new_s, caller, domain_id) == power_thresh_high)
  && ((IsValidPowercapDomain(old_s, domain_id) &&
       AreValidParameters(old_s, domain_id, notify_enable, power_thresh_low, power_thresh_high))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> MeasurementsNotifyEnabled(new_s, caller, domain_id) == MeasurementsNotifyEnabled(old_s, caller, domain_id))
  && (result != SUCCESS
    ==> PowerThreshLow(new_s, caller, domain_id) == PowerThreshLow(old_s, caller, domain_id))
  && (result != SUCCESS
    ==> PowerThreshHigh(new_s, caller, domain_id) == PowerThreshHigh(old_s, caller, domain_id))
}