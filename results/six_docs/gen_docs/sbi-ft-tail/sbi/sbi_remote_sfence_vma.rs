pub open spec fn sbi_remote_sfence_vma_spec(hart_mask: UInt64, hart_mask_base: UInt64, start_addr: UInt64, size: UInt64, result: SbiReturnCode, old_s: S, new_s: S) -> bool {
  ((start_addr) | (size) ==> result == SBI_ERR_INVALID_ADDRESS)
  && ((!(start_addr) && !(size)) ==> result == SBI_SUCCESS)
  && (result == SBI_ERR_INVALID_ADDRESS ==> hart_mask == hart_mask)
  && (result == SBI_ERR_INVALID_ADDRESS ==> hart_mask_base == hart_mask_base)
  && (result == SBI_ERR_INVALID_ADDRESS ==> start_addr == start_addr)
  && (result == SBI_ERR_INVALID_ADDRESS ==> size == size)
  && (result == SBI_SUCCESS ==> hart_mask == hart_mask)
  && (result == SBI_SUCCESS ==> hart_mask_base == hart_mask_base)
  && (result == SBI_SUCCESS ==> start_addr == start_addr)
  && (result == SBI_SUCCESS ==> size == size)
}