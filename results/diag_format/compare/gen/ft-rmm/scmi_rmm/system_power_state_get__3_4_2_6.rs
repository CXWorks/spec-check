pub open spec fn system_power_state_get__3_4_2_6_spec(system_state: UInt32, old_s: S, new_s: S) -> bool {
  (system_state == 0) || (system_state == 3) || (system_state == 4) || (system_state >= 0x80000000 && system_state <= 0xFFFFFFFF)
  && !(system_state >= 5 && system_state <= 0x7FFFFFFF)
  && true
}