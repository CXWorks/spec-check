pub open spec fn sbi_remote_hfence_gvma_spec(hart_mask: UInt64, hart_mask_base: UInt64, start_addr: UInt64, size: UInt64, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_ADDRESS ==> (start_addr >= (1 << 64) || size >= (1 << 64)))
  && (result == SBI_ERR_INVALID_PARAM ==> true)
  && ((!(start_addr >= (1 << 64)) &&
       !(size >= (1 << 64)))
    ==> result == SBI_SUCCESS)
  && (result == SBI_ERR_NOT_SUPPORTED ==> true)
  && (result == SBI_ERR_FAILED ==> true)
  && ((result == SBI_SUCCESS ||
       result == SBI_ERR_NOT_SUPPORTED ||
       result == SBI_ERR_INVALID_PARAM ||
       result == SBI_ERR_FAILED)
    ==> true)
}