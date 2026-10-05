pub open spec fn sbi_steal_time_set_shmem_spec(shmem_phys_lo: unsigned long, shmem_phys_hi: unsigned long, flags: unsigned long, result: struct sbiret, old_s: S, new_s: S) -> bool {
  (!IsAllOnes(old_s, shmem_phys_lo) || !IsAllOnes(old_s, shmem_phys_hi) ==> StealTimeShmemBase(new_s, calling_hart) == Concat(new_s, shmem_phys_hi, shmem_phys_lo))
  && (!IsAllOnes(old_s, shmem_phys_lo) || !IsAllOnes(old_s, shmem_phys_hi) ==> IsZero(new_s, Mem(Concat(new_s, shmem_phys_hi, shmem_phys_lo), 64)))
  && (!IsAllOnes(old_s, shmem_phys_lo) || !IsAllOnes(old_s, shmem_phys_hi) ==> StealTimeReportingEnabled(new_s, calling_hart) == true)
  && (IsAllOnes(old_s, shmem_phys_lo) && IsAllOnes(old_s, shmem_phys_hi) ==> StealTimeReportingEnabled(new_s, calling_hart) == false)
  && ((!(IsAllOnes(old_s, shmem_phys_lo) || !IsAllOnes(old_s, shmem_phys_hi))) ==> StealTimeShmemBase(new_s, calling_hart) == StealTimeShmemBase(old_s, calling_hart))
  && ((!(IsAllOnes(old_s, shmem_phys_lo) || !IsAllOnes(old_s, shmem_phys_hi))) ==> StealTimeReportingEnabled(new_s, calling_hart) == StealTimeReportingEnabled(old_s, calling_hart))
  && (result.code != 0 ==> StealTimeShmemBase(new_s, calling_hart) == StealTimeShmemBase(old_s, calling_hart))
  && (result.code != 0 ==> StealTimeReportingEnabled(new_s, calling_hart) == StealTimeReportingEnabled(old_s, calling_hart))
}