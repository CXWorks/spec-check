pub open spec fn sbi_sse_hart_unmask_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result == SBI_ERR_ALREADY_STARTED ==> old_s.sse_hart_unmasked(old_s.hart_id) && new_s.sse_hart_unmasked(new_s.hart_id))
    && (result == SBI_ERR_FAILED ==> !old_s.sse_hart_unmasked(old_s.hart_id) && !new_s.sse_hart_unmasked(new_s.hart_id))
    && (result == SBI_SUCCESS ==> !old_s.sse_hart_unmasked(old_s.hart_id) && new_s.sse_hart_unmasked(new_s.hart_id))
}