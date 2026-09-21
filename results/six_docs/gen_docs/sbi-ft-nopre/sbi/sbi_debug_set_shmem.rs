pub open spec fn sbi_debug_set_shmem_spec(shmem_phys_lo: UInt64, shmem_phys_hi: UInt64, flags: UInt64, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_PARAM && (flags != 0 || !(shmem_phys_lo % (64 / 8)) == 0))
  && (result == SBI_ERR_INVALID_ADDRESS)
  && (result == SBI_ERR_FAILED)
  && ((!(shmem_phys_lo == -1 && shmem_phys_hi == -1) &&
       (flags == 0) &&
       ((shmem_phys_lo % (64 / 8)) == 0))
    ==> result == SBI_SUCCESS)
  && (result == SBI_SUCCESS
    ==> flags == 0)
  && ((result != SBI_SUCCESS)
    ==> flags == 0)
}