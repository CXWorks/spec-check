pub open spec fn sbi_steal_time_set_shmem_spec(shmem_phys_lo: UInt, shmem_phys_hi: UInt, flags: UInt, old_s: S, new_s: S) -> bool {
  (!(IsAllOnes(shmem_phys_lo) && IsAllOnes(shmem_phys_hi)) ==> StealTimeShmemBase(new_s, CallingHart(new_s)) == ShmemPhysAddr(new_s, shmem_phys_lo, shmem_phys_hi))
  && (!(IsAllOnes(shmem_phys_lo) && IsAllOnes(shmem_phys_hi)) ==> StealTimeReportingEnabled(new_s, CallingHart(new_s)))
  && (!(IsAllOnes(shmem_phys_lo) && IsAllOnes(shmem_phys_hi)) ==> (forall i: UInt64| i < 64 ==> MemByte(new_s, ShmemPhysAddr(new_s, shmem_phys_lo, shmem_phys_hi) + i) == 0))
  && ((IsAllOnes(shmem_phys_lo) && IsAllOnes(shmem_phys_hi)) ==> !StealTimeReportingEnabled(new_s, CallingHart(new_s)))
  && ((!(IsAllOnes(shmem_phys_lo) && IsAllOnes(shmem_phys_hi))) ==> StealTimeShmemBase(old_s, CallingHart(old_s)) == StealTimeShmemBase(new_s, CallingHart(new_s)))
  && (StealTimeReportingEnabled(old_s, CallingHart(old_s)) ==> StealTimeReportingEnabled(new_s, CallingHart(new_s)))
  && ((!(IsAllOnes(shmem_phys_lo) && IsAllOnes(shmem_phys_hi))) ==> (forall i: UInt64| i < 64 ==> MemByte(old_s, ShmemPhysAddr(old_s, shmem_phys_lo, shmem_phys_hi) + i) == MemByte(new_s, ShmemPhysAddr(new_s, shmem_phys_lo, shmem_phys_hi) + i)))
  && (StealTimeReportingEnabled(old_s, CallingHart(old_s)) && !((IsAllOnes(shmem_phys_lo) && IsAllOnes(shmem_phys_hi)))) ==> StealTimeReportingEnabled(new_s, CallingHart(new_s))
}