pub open spec fn sbi_sse_hart_mask_spec(result: int, old_s: S, new_s: S) -> bool {
  (result == SBI_SUCCESS)
  && (result == SBI_ERR_ALREADY_STOPPED)
  && (result == SBI_ERR_FAILED)
  && ((!(result == SBI_SUCCESS) &&
       !(result == SBI_ERR_ALREADY_STOPPED) &&
       !(result == SBI_ERR_FAILED))
    ==> result == SBI_ERR_INVALID_PARAM)
}