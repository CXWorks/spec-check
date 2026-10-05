pub open spec fn system_power_state_notifier__3_4_3_1_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    // Reserved bits in flags must be zero
    ((old_s.flags >> 1) as int) == 0
    // system_state must be valid (0x0, 0x1, 0x2, 0x3, 0x4, or 0x80000000–0xFFFFFFFF)
    && (old_s.system_state == 0 || old_s.system_state == 1 || old_s.system_state == 2 || old_s.system_state == 3 || old_s.system_state == 4 || (old_s.system_state as int) >= 0x80000000)
    // timeout must be zero unless system_state is 0x0 and flags bit 0 is set
    && (old_s.system_state != 0 || (old_s.flags & 1) == 0 || old_s.timeout == 0)
    // Command must succeed (RSI_SUCCESS) if all preconditions are met
    && (result == RSI_SUCCESS)
}