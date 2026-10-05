pub open spec fn sbi_sse_hart_unmask_spec(result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_ALREADY_STARTED ==> true)
  && (result == SBI_ERR_FAILED ==> true)
  && (result == SBI_SUCCESS ==> true)
  && ((!(result == SBI_ERR_ALREADY_STARTED) &&
       !(result == SBI_ERR_FAILED))
    ==> result == SBI_SUCCESS)
}