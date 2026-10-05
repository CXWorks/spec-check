pub open spec fn sbi_nacl_set_shmem_spec(result: SbiErrorCode, old_s: S, new_s: S, shmem_phys_lo: UInt, shmem_phys_hi: UInt, flags: UInt) -> bool {
    (flags != 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (!(IsAllOnes(shmem_phys_lo) && IsAllOnes(shmem_phys_hi)) && (shmem_phys_lo as int) % 4096 != 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (!(IsAllOnes(shmem_phys_lo) && IsAllOnes(shmem_phys_hi)) && !ShmemSatisfiesRequirements(ShmemBase(shmem_phys_lo, shmem_phys_hi), 4096 + XLEN * 128) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
    && (RequestFailedForUnspecifiedReason() ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS))
    && (!(IsAllOnes(shmem_phys_lo) && IsAllOnes(shmem_phys_hi)) ==> (NaclShmemEnabled(new_s) && NaclShmemBase(new_s) == ShmemBase(shmem_phys_lo, shmem_phys_hi)))
    && ((IsAllOnes(shmem_phys_lo) && IsAllOnes(shmem_phys_hi)) ==> !NaclFeaturesEnabled(new_s))
}