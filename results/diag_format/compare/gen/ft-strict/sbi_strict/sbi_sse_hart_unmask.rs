pub open spec fn sbi_sse_hart_unmask_spec(result: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (SseHartUnmasked(old_s, CurrentHart()) ==> ResultEqual(result, SBI_ERR_ALREADY_STARTED))
  && (SseUnspecifiedFailure(old_s, CurrentHart()) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> SseHartUnmasked(new_s, CurrentHart()))
  && ((!SseHartUnmasked(old_s, CurrentHart()) &&
       !SseUnspecifiedFailure(old_s, CurrentHart()))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> SseHartUnmasked(new_s, CurrentHart()))
}