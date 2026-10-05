pub open spec fn sbi_debug_set_shmem_spec(error: i64, old_s: S, new_s: S) -> bool {
    (old_s.flags != 0 ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (!IsDisableRequest(old_s.shmem_phys_lo, old_s.shmem_phys_hi) && (old_s.shmem_phys_lo as int) % ((old_s.XLEN as int) / 8) != 0 ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (!IsDisableRequest(old_s.shmem_phys_lo, old_s.shmem_phys_hi) && !ShmemSatisfiesSection3_2Requirements(PhysAddr(old_s.shmem_phys_hi, old_s.shmem_phys_lo), TrigMax() * ((old_s.XLEN as int) / 2)) ==> ResultEqual(error, SBI_ERR_INVALID_ADDRESS))
    && RequestFailedForUnspecifiedReason() ==> ResultEqual(error, SBI_ERR_FAILED)
    && ResultEqual(error, SBI_SUCCESS)
    && (!IsDisableRequest(old_s.shmem_phys_lo, old_s.shmem_phys_hi) ==> (DebugShmemEnabled(new_s, CallingHart()) && DebugShmemBase(new_s, CallingHart()) == PhysAddr(old_s.shmem_phys_hi, old_s.shmem_phys_lo) && DebugShmemSize(new_s, CallingHart()) == TrigMax() * ((old_s.XLEN as int) / 2)))
    && (IsDisableRequest(old_s.shmem_phys_lo, old_s.shmem_phys_hi) ==> !DebugShmemEnabled(new_s, CallingHart()))
}