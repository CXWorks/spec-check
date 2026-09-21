pub open spec fn sbi_sse_hart_unmask_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_ALREADY_STARTED ==> true)
    && (result == SBI_SBI_ERR_FAILED ==> true)
    && (result == SBI_SBI_SUCCESS ==> true)
}