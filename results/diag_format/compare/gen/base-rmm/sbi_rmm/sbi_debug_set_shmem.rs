pub open spec fn sbi_debug_set_shmem_spec(error: sbiret, old_s: S, new_s: S) -> bool {
    (old_s.flags != 0 ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (!IsAllOnes(old_s.shmem_phys_lo, old_s.shmem_phys_hi)
        && !IsAligned(old_s.shmem_phys_lo, XLEN / 8)
        ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (!IsAllOnes(old_s.shmem_phys_lo, old_s.shmem_phys_hi)
        && !ShmemSatisfiesRequirements(old_s.shmem_phys_hi, old_s.shmem_phys_lo, trig_max * (XLEN / 2))
        ==> ResultEqual(error, SBI_ERR_INVALID_ADDRESS))
    && (RequestFailedForOtherReason() ==> ResultEqual(error, SBI_ERR_FAILED))
    && (ResultEqual(error, SBI_SUCCESS)
        ==> (!IsAllOnes(old_s.shmem_phys_lo, old_s.shmem_phys_hi)
            ==> (DebugShmem(CallingHart()).enabled == TRUE
                && DebugShmem(CallingHart()).base == (old_s.shmem_phys_hi << XLEN) | old_s.shmem_phys_lo
                && DebugShmem(CallingHart()).size == trig_max * (XLEN / 2)))
            && (IsAllOnes(old_s.shmem_phys_lo, old_s.shmem_phys_hi)
                ==> DebugShmem(CallingHart()).enabled == FALSE))
}