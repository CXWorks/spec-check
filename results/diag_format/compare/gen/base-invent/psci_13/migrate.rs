pub open spec fn migrate_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (old_s.target_cpu as int < 0 || old_s.target_cpu as int >= (1u64 << 32)))
    && (result == RSI_ERROR_STATE ==> (old_s.target_cpu as int != old_s.current_cpu))
    && (result == RSI_INCOMPLETE ==> true)
    && (result == RSI_ERROR_UNKNOWN ==> true)
    && (result == RSI_SUCCESS ==> (new_s.current_cpu == old_s.target_cpu))
}