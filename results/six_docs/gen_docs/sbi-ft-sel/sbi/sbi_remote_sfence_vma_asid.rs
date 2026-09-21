pub open spec fn sbi_remote_sfence_vma_asid_spec(hart_mask: UInt64, hart_mask_base: UInt64, start_addr: UInt64, size: UInt64, asid: UInt64, result: SbiRet, old_s: S, new_s: S) -> bool {
  ((start_addr) < 0 ==> SBI_ERR_INVALID_ADDRESS)
  && (size) < 0 ==> SBI_ERR_INVALID_ADDRESS
  && (asid) < 0 ==> SBI_ERR_INVALID_PARAM
  && (result == SBI_SUCCESS)
  && (result == SBI_ERR_INVALID_ADDRESS)
  && (result == SBI_ERR_INVALID_PARAM)
  && (result == SBI_ERR_FAILED)
  && ((!( (start_addr) < 0) &&
       !( (size) < 0) &&
       !( (asid) < 0))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> result == SBI_ERR_INVALID_ADDRESS)
  && (result != SBI_SUCCESS
    ==> result == SBI_ERR_INVALID_PARAM)
  && (result != SBI_SUCCESS
    ==> result == SBI_ERR_FAILED)
}