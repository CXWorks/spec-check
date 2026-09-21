pub open spec fn sbi_sse_hart_mask_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_ALREADY_STOPPED ==> new_s == old_s)
    && (result == SBI_SBI_ERR_FAILED ==> new_s == old_s)
    && (result == SBI_SBI_SUCCESS ==> new_s != old_s)
}