pub open spec fn node_hw_state_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (old_s.target_cpu as int < 0 || old_s.target_cpu as int >= (1u64 << 48) || old_s.power_level as int < 0))
    && (result == RSI_SUCCESS ==> (old_s.target_cpu as int >= 0 && old_s.target_cpu as int < (1u64 << 48) && old_s.power_level as int >= 0))
}