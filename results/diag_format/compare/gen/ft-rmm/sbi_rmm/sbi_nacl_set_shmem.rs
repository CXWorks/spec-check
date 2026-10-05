pub open spec fn sbi_nacl_set_shmem_spec(shmem_phys_lo: Address, shmem_phys_hi: Address, flags: UInt, result: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (flags != 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (!IsAllOnes(old_s, shmem_phys_lo, shmem_phys_hi) && !AddrIsAligned(old_s, shmem_phys_lo, 4096) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (!IsAllOnes(old_s, shmem_phys_lo, shmem_phys_hi) && !ShmemSatisfiesRequirements(old_s, shmem_phys_hi:shmem_phys_lo, 4096 + (64 * 128)) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
  && (RequestFailedForUnspecifiedReason(old_s) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> !IsAllOnes(new_s, shmem_phys_lo, shmem_phys_hi) ==> NaclShmemBase(new_s, CallingHart()) == shmem_phys_hi:shmem_phys_lo && NaclEnabled(new_s, CallingHart()))
  && (result == SBI_SUCCESS ==> IsAllOnes(new_s, shmem_phys_lo, shmem_phys_hi) ==> !NaclEnabled(new_s, CallingHart()))
  && ((!(flags != 0) &&
       (IsAllOnes(old_s, shmem_phys_lo, shmem_phys_hi) || AddrIsAligned(old_s, shmem_phys_lo, 4096)) &&
       (IsAllOnes(old_s, shmem_phys_lo, shmem_phys_hi) || ShmemSatisfiesRequirements(old_s, shmem_phys_hi:shmem_phys_lo, 4096 + (64 * 128))) &&
       !(RequestFailedForUnspecifiedReason(old_s)))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> NaclShmemBase(new_s, CallingHart()) == NaclShmemBase(old_s, CallingHart()))
  && (result != SBI_SUCCESS
    ==> NaclEnabled(new_s, CallingHart()) == NaclEnabled(old_s, CallingHart()))
}