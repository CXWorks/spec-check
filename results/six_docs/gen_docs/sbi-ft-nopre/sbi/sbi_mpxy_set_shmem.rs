pub open spec fn sbi_mpxy_set_shmem_spec(shmem_phys_lo: UInt64, shmem_phys_hi: UInt64, flags: UInt64, old_s: S, new_s: S) -> bool {
  ((shmem_phys_lo != -1 && shmem_phys_hi != -1) && !((shmem_phys_lo & 4095) == 0) ==> false)
  && ((shmem_phys_lo == -1 && shmem_phys_hi == -1) ==> true)
  && ((!(shmem_phys_lo != -1 && shmem_phys_hi != -1) && (shmem_phys_lo == -1 || shmem_phys_hi == -1)) ==> true)
}