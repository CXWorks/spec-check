pub open spec fn sbi_nacl_set_shmem_spec(result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (old_s.flags != 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (!IsAllOnes(old_s.shmem_phys_lo, old_s.shmem_phys_hi) && !AddrIsAligned(old_s.shmem_phys_lo, 4096) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (!IsAllOnes(old_s.shmem_phys_lo, old_s.shmem_phys_hi) && !ShmemSatisfiesRequirements(old_s.shmem_phys_hi:old_s.shmem_phys_lo, 4096 + (XLEN * 128)) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
    && (RequestFailedForUnspecifiedReason() ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> (!IsAllOnes(old_s.shmem_phys_lo, old_s.shmem_phys_hi) ==> (NaclShmemBase(CallingHart()) == old_s.shmem_phys_hi:old_s.shmem_phys_lo && NaclEnabled(CallingHart()))))
    && (ResultEqual(result, SBI_SUCCESS) ==> (IsAllOnes(old_s.shmem_phys_lo, old_s.shmem_phys_hi) ==> !NaclEnabled(CallingHart())))
}