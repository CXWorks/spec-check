pub open spec fn 3.3.2.8_power_state_notify_spec(domain_id: UInt32, notify_enable: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS)
}