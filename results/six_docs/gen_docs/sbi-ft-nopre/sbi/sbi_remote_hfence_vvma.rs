pub open spec fn sbi_remote_hfence_vvma_spec(hart_mask: UInt64, hart_mask_base: UInt64, start_addr: UInt64, size: UInt64, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_ADDRESS ==> true)
  && (result == SBI_ERR_INVALID_PARAM ==> true)
  && (result == SBI_ERR_FAILED ==> true)
  && ((!(result == SBI_ERR_NOT_SUPPORTED) &&
       result == SBI_SUCCESS)
    ==> true)
  && ((!(result == SBI_ERR_NOT_SUPPORTED) &&
       result != SBI_SUCCESS)
    ==> true)
}