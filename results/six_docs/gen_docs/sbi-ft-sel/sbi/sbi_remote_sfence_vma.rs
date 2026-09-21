pub open spec fn sbi_remote_sfence_vma_spec(hart_mask: UInt64, hart_mask_base: UInt64, start_addr: UInt64, size: UInt64, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_ADDRESS ==> true)
  && (result == SBI_SUCCESS ==> true)
  && ((!( (start_addr) < 0) &&
       !( (size) < 0))
    ==> result == SBI_SUCCESS)
}