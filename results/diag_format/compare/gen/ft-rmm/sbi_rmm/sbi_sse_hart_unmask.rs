pub open spec fn sbi_sse_hart_unmask_spec(result: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (!SseEventsMaskedOnHart(old_s, CallingHart(old_s)) ==> ResultEqual(result, SBI_ERR_ALREADY_STARTED))
  && (RequestFailedForUnspecifiedReason(old_s) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> !SseEventsMaskedOnHart(new_s, CallingHart(new_s)))
  && ((!(SseEventsMaskedOnHart(old_s, CallingHart(old_s))) &&
       !(RequestFailedForUnspecifiedReason(old_s)))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> SseEventsMaskedOnHart(new_s, CallingHart(new_s)))
}