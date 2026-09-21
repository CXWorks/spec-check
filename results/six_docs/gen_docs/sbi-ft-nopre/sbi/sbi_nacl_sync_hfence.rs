pub open spec fn sbi_nacl_sync_hfence_spec(entry_index: UInt64, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_PARAM && (entry_index != -1 && entry_index >= (3840 / XLEN)) ==> true)
  && ((!(result == SBI_ERR_INVALID_PARAM) &&
       (result == SBI_ERR_INVALID_PARAM ||
        result == SBI_ERR_NOT_SUPPORTED ||
        result == SBI_ERR_NO_SHMEM))
    ==> true)
  && (result == SBI_SUCCESS
    ==> true)
  && ((!(result == SBI_ERR_INVALID_PARAM) &&
       result != SBI_ERR_NOT_SUPPORTED &&
       result != SBI_ERR_NO_SHMEM)
    ==> true)
}