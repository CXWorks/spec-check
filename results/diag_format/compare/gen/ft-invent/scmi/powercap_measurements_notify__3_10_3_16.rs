pub open spec fn powercap_measurements_notify__3_10_3_16_spec(domain_id: UInt32, notify_enable: UInt32, power_thresh_low: UInt32, power_thresh_high: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS)
  && (result == RSI_ERROR_NOT_FOUND)
  && (result == RSI_ERROR_INVALID_PARAMETERS)
}