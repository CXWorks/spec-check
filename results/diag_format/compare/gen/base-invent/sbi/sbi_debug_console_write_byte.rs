pub open spec fn sbi_debug_console_write_byte_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_DENIED ==> true)
    && (result.error == SBI_ERR_FAILED ==> true)
    && (result.error == SBI_SUCCESS ==> true)
}