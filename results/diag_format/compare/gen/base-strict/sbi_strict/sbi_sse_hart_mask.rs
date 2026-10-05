pub open spec fn sbi_sse_hart_mask_spec(result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (SseEventsMasked(old_s) ==> ResultEqual(result, SBI_ERR_ALREADY_STOPPED))
    && (RequestFailedForUnspecifiedReason(old_s) ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> SseEventsMasked(new_s))
    && (ResultEqual(result, SBI_SUCCESS) ==> !SseHartReadyToReceiveEvents(new_s))
}