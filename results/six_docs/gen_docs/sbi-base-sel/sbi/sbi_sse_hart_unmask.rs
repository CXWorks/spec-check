pub open spec fn sbi_sse_hart_unmask_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_ALREADY_STARTED ==> old_s.sse_hart_unmasked(old_s.hart_id) == true)
    && (result == SBI_SBI_ERR_FAILED ==> true)
    && (result == SBI_SBI_SUCCESS ==> old_s.sse_hart_unmasked(old_s.hart_id) == false && new_s.sse_hart_unmasked(new_s.hart_id) == true)
}