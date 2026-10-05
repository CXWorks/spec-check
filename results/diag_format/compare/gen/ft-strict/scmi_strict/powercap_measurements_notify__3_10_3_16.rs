pub open spec fn powercap_measurements_notify__3_10_3_16_spec(domain_id: UInt32, notify_enable: UInt32, power_thresh_low: UInt32, power_thresh_high: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!AreValidMeasurementsNotifyParameters(old_s, domain_id, notify_enable, power_thresh_low, power_thresh_high) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> MeasurementsNotifyEnabled(new_s, CallingAgent(new_s), domain_id) == Bits(notify_enable, 0, 0))
  && (ResultEqual(status, SUCCESS) ==> PowerThresholdLow(new_s, CallingAgent(new_s), domain_id) == power_thresh_low)
  && (ResultEqual(status, SUCCESS) ==> PowerThresholdHigh(new_s, CallingAgent(new_s), domain_id) == power_thresh_high)
  && (ResultEqual(status, SUCCESS) ==> Bits(notify_enable, 0, 0) == 1 && AveragePowerOverMai(new_s, domain_id) < power_thresh_low ==> SendsMeasurementsChanged(new_s, CallingAgent(new_s), domain_id))
  && (ResultEqual(status, SUCCESS) ==> Bits(notify_enable, 0, 0) == 1 && AveragePowerOverMai(new_s, domain_id) > power_thresh_high ==> SendsMeasurementsChanged(new_s, CallingAgent(new_s), domain_id))
  && (ResultEqual(status, SUCCESS) ==> Bits(notify_enable, 0, 0) == 1 && MaiChanged(new_s, domain_id) ==> SendsMeasurementsChanged(new_s, CallingAgent(new_s), domain_id))
  && (ResultEqual(status, SUCCESS) ==> Bits(notify_enable, 0, 0) == 0 ==> !SendsMeasurementsChanged(new_s, CallingAgent(new_s), domain_id))
  && ((IsValidPowercapDomain(old_s, domain_id) &&
       AreValidMeasurementsNotifyParameters(old_s, domain_id, notify_enable, power_thresh_low, power_thresh_high))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> MeasurementsNotifyEnabled(new_s, CallingAgent(new_s), domain_id) == MeasurementsNotifyEnabled(old_s, CallingAgent(old_s), domain_id))
  && (result != SUCCESS
    ==> PowerThresholdLow(new_s, CallingAgent(new_s), domain_id) == PowerThresholdLow(old_s, CallingAgent(old_s), domain_id))
  && (result != SUCCESS
    ==> PowerThresholdHigh(new_s, CallingAgent(new_s), domain_id) == PowerThresholdHigh(old_s, CallingAgent(old_s), domain_id))
  && (result != SUCCESS
    ==> AveragePowerOverMai(new_s, domain_id) == AveragePowerOverMai(old_s, domain_id))
  && (result != SUCCESS
    ==> MaiChanged(new_s, domain_id) == MaiChanged(old_s, domain_id))
}