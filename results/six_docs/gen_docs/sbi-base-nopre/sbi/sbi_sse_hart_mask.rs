pub open spec fn sbi_sse_hart_mask_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result == SBI_ERR_ALREADY_STOPPED ==> (old_s.hart_masked == true))
    && (result == SBI_ERR_FAILED ==> (old_s.hart_masked == false))
    && (result == SBI_SUCCESS ==> (old_s.hart_masked == false && new_s.hart_masked == true))
}