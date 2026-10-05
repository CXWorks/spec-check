pub open spec fn sbi_debug_set_shmem_spec(shmem_phys_lo: unsigned long, shmem_phys_hi: unsigned long, flags: unsigned long, error: long, old_s: S, new_s: S) -> bool {
  (flags != 0 ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (!IsDisableRequest(old_s, shmem_phys_lo, shmem_phys_hi) && (shmem_phys_lo % (XLEN(old_s) / 8)) != 0 ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (!IsDisableRequest(old_s, shmem_phys_lo, shmem_phys_hi) && !ShmemSatisfiesSection3_2Requirements(old_s, PhysAddr(old_s, shmem_phys_hi, shmem_phys_lo), TrigMax(old_s) * (XLEN(old_s) / 2)) ==> ResultEqual(error, SBI_ERR_INVALID_ADDRESS))
  && (RequestFailedForUnspecifiedReason(old_s) ==> ResultEqual(error, SBI_ERR_FAILED))
  && (ResultEqual(error, SBI_SUCCESS) ==> !IsDisableRequest(old_s, shmem_phys_lo, shmem_phys_hi) ==> DebugShmemEnabled(new_s, CallingHart(new_s)) && DebugShmemBase(new_s, CallingHart(new_s)) == PhysAddr(new_s, shmem_phys_hi, shmem_phys_lo) && DebugShmemSize(new_s, CallingHart(new_s)) == TrigMax(new_s) * (XLEN(new_s) / 2))
  && (ResultEqual(error, SBI_SUCCESS) ==> IsDisableRequest(old_s, shmem_phys_lo, shmem_phys_hi) ==> !DebugShmemEnabled(new_s, CallingHart(new_s)))
  && ((!(flags != 0) &&
       (IsDisableRequest(old_s, shmem_phys_lo, shmem_phys_hi) || ((shmem_phys_lo % (XLEN(old_s) / 8))) == 0) &&
       (IsDisableRequest(old_s, shmem_phys_lo, shmem_phys_hi) || ShmemSatisfiesSection3_2Requirements(old_s, PhysAddr(old_s, shmem_phys_hi, shmem_phys_lo), TrigMax(old_s) * (XLEN(old_s) / 2))) &&
       !RequestFailedForUnspecifiedReason(old_s))
    ==> ResultEqual(error, SBI_SUCCESS))
  && (result != SBI_SUCCESS
    ==> DebugShmemEnabled(new_s, CallingHart(new_s)) == DebugShmemEnabled(old_s, CallingHart(old_s)))
  && (result != SBI_SUCCESS
    ==> DebugShmemBase(new_s, CallingHart(new_s)) == DebugShmemBase(old_s, CallingHart(old_s)))
  && (result != SBI_SUCCESS
    ==> DebugShmemSize(new_s, CallingHart(new_s)) == DebugShmemSize(old_s, CallingHart(old_s)))
}