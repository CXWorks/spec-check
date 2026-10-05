pub open spec fn sbi_fwft_get_spec(feature: UInt32, result: SbiRet, old_s: S, new_s: S) -> bool {
  (result.error == SBI_ERR_NOT_SUPPORTED ==> result.value == 0)
  && (result.error == SBI_ERR_DENIED ==> result.value == 0)
  && (result.error == SBI_ERR_FAILED ==> result.value == 0)
  && ((!(result.error == SBI_ERR_NOT_SUPPORTED) &&
       !(result.error == SBI_ERR_DENIED) &&
       !(result.error == SBI_ERR_FAILED))
    ==> result.value == result.value)
}