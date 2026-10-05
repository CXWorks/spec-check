pub open spec fn sbi_sse_hart_mask_spec(eid: UInt, fid: UInt, result: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (SseEventsMasked(old_s, current_hart(old_s)) ==> ResultEqual(result, SBI_ERR_ALREADY_STOPPED))
  && (RequestFailedForUnspecifiedReason(old_s, current_hart(old_s)) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> SseEventsMasked(new_s, current_hart(new_s)))
  && (result == SBI_SUCCESS ==> !SseHartReadyToReceiveEvents(new_s, current_hart(new_s)))
  && ((!SseEventsMasked(old_s, current_hart(old_s)) &&
       !RequestFailedForUnspecifiedReason(old_s, current_hart(old_s)))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> SseEventsMasked(new_s, current_hart(new_s)))
  && (result != SBI_SUCCESS
    ==> SseHartReadyToReceiveEvents(new_s, current_hart(new_s)))
}