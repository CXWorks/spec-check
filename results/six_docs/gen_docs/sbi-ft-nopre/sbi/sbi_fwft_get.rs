pub open spec fn sbi_fwft_get_spec(feature: UInt32, result: SbiRet, old_s: S, new_s: S) -> bool {
  (result.error != SBI_SUCCESS ==> result.value == 0)
  && ((result.error == SBI_SUCCESS)
    ==> result.value != 0)
}