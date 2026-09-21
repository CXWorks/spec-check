pub open spec fn sbi_cppc_read_hi_spec(cppc_reg_id: UInt32, result: sbiret, old_s: S, new_s: S) -> bool {
  (result.error == SBI_ERR_INVALID_PARAM ==> result.value == 0)
  && (result.error == SBI_ERR_NOT_SUPPORTED ==> result.value == 0)
  && (result.error == SBI_ERR_DENIED ==> result.value == 0)
  && (result.error == SBI_ERR_FAILED ==> result.value == 0)
  && ((!(result.error == SBI_ERR_INVALID_PARAM) &&
       !(result.error == SBI_ERR_NOT_SUPPORTED) &&
       !(result.error == SBI_ERR_DENIED) &&
       !(result.error == SBI_ERR_FAILED))
    ==> result.value == 0)
}