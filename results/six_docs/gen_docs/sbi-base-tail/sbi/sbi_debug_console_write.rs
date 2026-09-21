pub open spec fn sbi_debug_console_write_spec(result: int, num_bytes: UInt64, base_addr_lo: UInt64, base_addr_hi: UInt64, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (num_bytes == 0 || base_addr_lo == 0 || base_addr_hi == 0))
    && (result == SBI_SBI_ERR_DENIED ==> true)
    && (result == SBI_SBI_ERR_FAILED ==> true)
    && (result == SBI_SBI_SUCCESS ==> true)
    && (result != SBI_SBI_SUCCESS ==> true)
}