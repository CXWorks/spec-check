pub open spec fn sbi_cppc_read_hi_spec(cppc_reg_id: UInt32, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_PARAM ==> true)
  && (result == SBI_ERR_NOT_SUPPORTED ==> true)
  && (result == SBI_ERR_DENIED ==> true)
  && (result == SBI_ERR_FAILED ==> true)
  && (result == SBI_SUCCESS ==> true)
  && ((!(result == SBI_ERR_INVALID_PARAM) &&
       result == SBI_SUCCESS)
    ==> true)
}