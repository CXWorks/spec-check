pub open spec fn sbi_steal_time_set_shmem_spec(result: struct sbiret, old_s: S, new_s: S, shmem_phys_lo: UInt, shmem_phys_hi: UInt, flags: UInt) -> bool {
    let is_all_ones = (shmem_phys_lo == (UInt::MAX)) && (shmem_phys_hi == (UInt::MAX));
    let shmem_phys_addr = ShmemPhysAddr(shmem_phys_lo, shmem_phys_hi);
    let calling_hart = CallingHart();
    let success = !is_all_ones;
    let error = result.error;
    let value = result.value;
    (!IsAllOnes(shmem_phys_lo) || !IsAllOnes(shmem_phys_hi) ==> error == 0)
    && (IsAllOnes(shmem_phys_lo) && IsAllOnes(shmem_phys_hi) ==> error == 0)
    && (success ==> (StealTimeShmemBase(new_s, calling_hart) == shmem_phys_addr))
    && (success ==> StealTimeReportingEnabled(new_s, calling_hart))
    && (success ==> (forall|i: UInt64| i < 64 ==> MemByte(new_s, shmem_phys_addr + i) == 0))
    && (is_all_ones ==> !StealTimeReportingEnabled(new_s, calling_hart))
    && (flags == 0)
    && (forall|reg: Register| reg != a0 && reg != a1 && reg != a2 ==> RegUnchanged(old_s, reg, new_s))
}