pub open spec fn sbi_debug_console_write_byte_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_DENIED ==> <unconstrained>)
    && (result.error == SBI_ERR_FAILED ==> <unconstrained>)
    && (result.error == SBI_SUCCESS ==> <unconstrained>)
}