pub open spec fn sbi_nacl_set_shmem_spec(shmem_phys_lo: UInt64, shmem_phys_hi: UInt64, flags: UInt64, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_ERR_INVALID_PARAM ==> flags == 0)
  && (result == SBI_ERR_INVALID_PARAM ==> shmem_phys_lo % 4096 == 0)
  && (result == SBI_SUCCESS && (shmem_phys_lo != -1 || shmem_phys_hi != -1) ==> true)
  && ((!(result == SBI_ERR_INVALID_PARAM) &&
       result == SBI_SUCCESS)
    ==> true)
  && (result != SBI_SUCCESS
    ==> RealmAt(new_s, 0).nacl_shmem_phys_lo == RealmAt(old_s, 0).nacl_shmem_phys_lo)
  && (result != SBI_SUCCESS
    ==> RealmAt(new_s, 0).nacl_shmem_phys_hi == RealmAt(old_s, 0).nacl_shmem_phys_hi)
}