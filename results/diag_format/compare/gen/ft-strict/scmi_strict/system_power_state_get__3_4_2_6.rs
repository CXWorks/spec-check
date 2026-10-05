pub open spec fn system_power_state_get__3_4_2_6_spec(status: Int32, system_state: UInt32, old_s: S, new_s: S) -> bool {
  (IsSuccessStatus(status) ==> system_state == CurrentSystemPowerState(new_s))
  && (system_state == 0x0 || system_state == 0x3 || system_state == 0x4 || system_state >= 0x80000000)
  && ((!(IsSuccessStatus(status)))
    ==> system_state == CurrentSystemPowerState(old_s))
}