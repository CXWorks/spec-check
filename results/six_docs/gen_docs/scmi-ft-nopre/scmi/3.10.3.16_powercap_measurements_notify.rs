pub open spec fn 3.10.3.16_powercap_measurements_notify_spec(domain_id: UInt32, notify_enable: UInt32, power_thresh_low: UInt32, power_thresh_high: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS)
  && (result == RSI_ERROR_NOT_FOUND)
  && (result == RSI_ERROR_INVALID_PARAMETERS)
}