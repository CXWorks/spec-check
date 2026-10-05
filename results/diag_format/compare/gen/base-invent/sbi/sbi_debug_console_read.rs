pub open spec fn sbi_debug_console_read_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error() ==> (result.error_code() == SBI_ERR_INVALID_PARAM || result.error_code() == SBI_ERR_INVALID_ADDRESS))
    && (result.error() == false ==> (new_s.console_buffer == old_s.console_buffer))
}