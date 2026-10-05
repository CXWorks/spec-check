pub open spec fn sbi_debug_set_shmem_spec(shmem_phys_lo: unsigned long, shmem_phys_hi: unsigned long, flags: unsigned long, result: sbiret, old_s: S, new_s: S) -> bool {
  (flags != 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (!IsAllOnes(old_s, shmem_phys_lo, shmem_phys_hi) && !IsAligned(old_s, shmem_phys_lo, XLEN / 8) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (!IsAllOnes(old_s, shmem_phys_lo, shmem_phys_hi) && !ShmemSatisfiesRequirements(old_s, shmem_phys_hi:shmem_phys_lo, trig_max * (XLEN / 2)) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
  && (RequestFailedForOtherReason(old_s) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> !IsAllOnes(new_s, shmem_phys_lo, shmem_phys_hi) ==> DebugShmem(new_s, CallingHart(new_s)).enabled == true && DebugShmem(new_s, CallingHart(new_s)).base == shmem_phys_hi:shmem_phys_lo && DebugShmem(new_s, CallingHart(new_s)).size == trig_max * (XLEN / 2))
  && (result == SBI_SUCCESS ==> IsAllOnes(new_s, shmem_phys_lo, shmem_phys_hi) ==> DebugShmem(new_s, CallingHart(new_s)).enabled == false)
  && ((!(flags != 0) &&
       (IsAllOnes(old_s, shmem_phys_lo, shmem_phys_hi) || IsAligned(old_s, shmem_phys_lo, XLEN / 8)) &&
       (IsAllOnes(old_s, shmem_phys_lo, shmem_phys_hi) || ShmemSatisfiesRequirements(old_s, shmem_phys_hi:shmem_phys_lo, trig_max * (XLEN / 2))) &&
       !(RequestFailedForOtherReason(old_s)))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> DebugShmem(new_s, CallingHart(new_s)).enabled == DebugShmem(old_s, CallingHart(old_s)).enabled)
  && (result != SBI_SUCCESS
    ==> DebugShmem(new_s, CallingHart(new_s)).base == DebugShmem(old_s, CallingHart(old_s)).base)
  && (result != SBI_SUCCESS
    ==> DebugShmem(new_s, CallingHart(new_s)).size == DebugShmem(old_s, CallingHart(old_s)).size)
}