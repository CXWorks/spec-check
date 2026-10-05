pub open spec fn sbi_sse_hart_unmask_spec(result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (!SseHartUnmasked(old_s) ==> ResultEqual(result, SBI_SUCCESS) && SseHartUnmasked(new_s))
    && (SseHartUnmasked(old_s) ==> ResultEqual(result, SBI_ERR_ALREADY_STARTED))
    && (SseUnspecifiedFailure(old_s) ==> ResultEqual(result, SBI_ERR_FAILED))
}