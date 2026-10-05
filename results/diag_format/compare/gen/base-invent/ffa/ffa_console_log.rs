pub open spec fn ffa_console_log_spec(result: int, old_s: S, new_s: S) -> bool {
    let count = (old_s.cmd_input_1 as int) & 0xFF;
    let is_sm32 = (old_s.cmd_input_0 as int) == 0x8400008A;
    let is_sm64 = (old_s.cmd_input_0 as int) == 0xC400008A;
    let max_count_sm32 = 24;
    let max_count_sm64 = 128;
    let max_count = if is_sm32 then max_count_sm32 else max_count_sm64;
    let is_invalid_params = (count == 0) || (count > max_count);
    let is_not_supported = false; // Assumed supported for spec unless context defines otherwise
    let is_retry = false; // Assumed success unless context defines retry conditions
    let success = !is_invalid_params && !is_not_supported && !is_retry;
    (is_invalid_params ==> result == FFA_ERROR_INVALID_PARAMETERS)
    && (is_not_supported ==> result == FFA_ERROR_NOT_SUPPORTED)
    && (is_retry ==> result == FFA_ERROR_RETRY)
    && (success ==> result == FFA_SUCCESS)
}