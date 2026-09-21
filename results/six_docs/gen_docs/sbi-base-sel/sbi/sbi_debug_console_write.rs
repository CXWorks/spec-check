pub open spec fn sbi_debug_console_write_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (old_s.num_bytes as int > 0 || (old_s.base_addr_lo as int < 0 || old_s.base_addr_hi as int < 0)))
    && (result == SBI_SBI_ERR_DENIED ==> true)
    && (result == SBI_SBI_ERR_FAILED ==> true)
    && (result == SBI_SBI_SUCCESS ==> (new_s.num_bytes as int <= old_s.num_bytes as int))
    && (result != SBI_SBI_SUCCESS && result != SBI_SBI_ERR_INVALID_PARAM && result != SBI_SBI_ERR_DENIED && result != SBI_SBI_ERR_FAILED ==> true)
}