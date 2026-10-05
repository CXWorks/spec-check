pub open spec fn sbi_sse_hart_mask_spec(result: SbiError, old_s: S, new_s: S) -> bool {
    (SseHartMasked(old_s, CurrentHartId(old_s)) ==> (result == SBI_ERR_ALREADY_STOPPED || result == SBI_ERR_FAILED))
    && (!SseHartMasked(old_s, CurrentHartId(old_s)) ==> (result == SBI_SUCCESS || result == SBI_ERR_FAILED))
    && (result == SBI_SUCCESS ==> (SseHartMasked(new_s, CurrentHartId(old_s)) && CurrentHartId(new_s) == CurrentHartId(old_s)))
    && (result != SBI_SUCCESS ==> new_s == old_s)
}
