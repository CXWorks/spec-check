pub open spec fn sbi_steal_time_set_shmem_spec(shmem_phys_lo: UInt64, shmem_phys_hi: UInt64, flags: UInt64, result: Result<(), SbiStatusCode>, old_s: S, new_s: S) -> bool {
  (result == SBI_SBI_ERR_INVALID_PARAM ==> flags == 0)
  && ((!(shmem_phys_lo == -1) && !(shmem_phys_hi == -1)) ==> (shmem_phys_lo % 64) == 0)
  && (result == SBI_SBI_SUCCESS ==> flags == 0)
  && ((!(result == SBI_SBI_SUCCESS) && result != SBI_SBI_ERR_INVALID_PARAM)
    ==> flags == 0)
}