pub open spec fn sbi_debug_console_write_byte_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_DENIED ==> true)
    && (result == SBI_SBI_ERR_FAILED ==> true)
    && (result == SBI_SBI_SUCCESS ==> true)
}