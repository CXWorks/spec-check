pub open spec fn sbi_nacl_set_shmem_spec(shmem_phys_lo: unsigned long, shmem_phys_hi: unsigned long, flags: unsigned long, result: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (flags != 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (!(IsAllOnes(old_s, shmem_phys_lo) && IsAllOnes(old_s, shmem_phys_hi)) && shmem_phys_lo % 4096 != 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (!(IsAllOnes(old_s, shmem_phys_lo) && IsAllOnes(old_s, shmem_phys_hi)) && !ShmemSatisfiesRequirements(old_s, ShmemBase(old_s, shmem_phys_lo, shmem_phys_hi), 4096 + 128) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
  && (RequestFailedForUnspecifiedReason(old_s) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (ResultEqual(result, SBI_SUCCESS) ==> !(IsAllOnes(new_s, shmem_phys_lo) && IsAllOnes(new_s, shmem_phys_hi)) ==> (NaclShmemEnabled(new_s, CurrentHart(new_s)) && NaclShmemBase(new_s, CurrentHart(new_s)) == ShmemBase(new_s, shmem_phys_lo, shmem_phys_hi)))
  && (ResultEqual(result, SBI_SUCCESS) && (IsAllOnes(new_s, shmem_phys_lo) && IsAllOnes(new_s, shmem_phys_hi)) ==> !NaclFeaturesEnabled(new_s, CurrentHart(new_s)))
  && ((!(flags != 0) &&
       !((!(IsAllOnes(old_s, shmem_phys_lo) && IsAllOnes(old_s, shmem_phys_hi)) && shmem_phys_lo % 4096 != 0)) &&
       !((!(IsAllOnes(old_s, shmem_phys_lo) && IsAllOnes(old_s, shmem_phys_hi)) && !ShmemSatisfiesRequirements(old_s, ShmemBase(old_s, shmem_phys_lo, shmem_phys_hi), 4096 + 128))) &&
       !RequestFailedForUnspecifiedReason(old_s))
    ==> ResultEqual(result, SBI_SUCCESS))
  && (result != SBI_SUCCESS
    ==> NaclShmemEnabled(new_s, CurrentHart(new_s)) == NaclShmemEnabled(old_s, CurrentHart(old_s)))
  && (result != SBI_SUCCESS
    ==> NaclShmemBase(new_s, CurrentHart(new_s)) == NaclShmemBase(old_s, CurrentHart(old_s)))
  && (result != SBI_SUCCESS
    ==> NaclFeaturesEnabled(new_s, CurrentHart(new_s)) == NaclFeaturesEnabled(old_s, CurrentHart(old_s)))
}