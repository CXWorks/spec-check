pub open spec fn sbi_sse_hart_mask_spec(error: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (HartSseMasked(old_s) ==> ResultEqual(error, SBI_ERR_ALREADY_STOPPED))
    && (RequestFailedForOtherReason(old_s) ==> ResultEqual(error, SBI_ERR_FAILED))
    && (ResultEqual(error, SBI_SUCCESS) ==> HartSseMasked(new_s))
}