pub open spec fn ffa_console_log_spec(old_s: S, new_s: S, function_id: UInt32, character_count: UInt32, character_regs: Seq<UInt64>, ret_function_id: UInt32, error_code: i32, logged_count: UInt32) -> bool {
    let count = (character_count & 0xffu32) as int;
    let is_smc32 = function_id == 0x8400008Au32;
    let is_smc64 = function_id == 0xC400008Au32;
    let count_invalid = count == 0 || (is_smc32 && count > 24) || (is_smc64 && count > 128);
    let is_error = ret_function_id == FFA_ERROR;
    let is_success = ret_function_id == FFA_SUCCESS;
    (!IsFfaConsoleLogImplemented(old_s) ==> (is_error && error_code == NOT_SUPPORTED))
    && ((IsFfaConsoleLogImplemented(old_s) && count_invalid) ==> (is_error && error_code == INVALID_PARAMETERS))
    && ((is_error && error_code == RETRY) ==> ((logged_count as int) < count && ConsoleCharactersLogged(old_s, new_s, character_regs, logged_count as int)))
    && ((is_error && error_code != RETRY) ==> (logged_count == 0 && new_s == old_s))
    && ((IsFfaConsoleLogImplemented(old_s) && (is_smc32 || is_smc64) && !count_invalid) ==> (is_success || (is_error && error_code == RETRY)))
    && (is_success ==> ConsoleCharactersLogged(old_s, new_s, character_regs, count))
}
