pub open spec fn sbi_sse_hart_unmask_spec(result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (!SseEventsMaskedOnHart(old_s, CallingHart()) ==> ResultEqual(result, SBI_ERR_ALREADY_STARTED))
    && (RequestFailedForUnspecifiedReason() ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> !SseEventsMaskedOnHart(new_s, CallingHart()))
    && (ResultEqual(result, SBI_SUCCESS) ==> SseEventsMaskedOnHart(old_s, CallingHart()))
}