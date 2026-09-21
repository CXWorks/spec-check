pub open spec fn sbi_debug_console_write_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> (num_bytes as int < 0 || base_addr_lo as int < 0 || base_addr_hi as int < 0))
    && (result.error == SBI_ERR_DENIED ==> true)
    && (result.error == SBI_ERR_FAILED ==> true)
    && (result.error == SBI_SUCCESS ==> (result.uvalue as int >= 0))
}