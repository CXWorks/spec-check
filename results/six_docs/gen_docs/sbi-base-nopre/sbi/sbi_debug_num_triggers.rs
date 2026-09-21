pub open spec fn sbi_debug_num_triggers_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error() ==> result.error_code() == SBI_ERR_INVALID_PARAM)
    && (result.error() == false ==> result.error_code() == 0)
}