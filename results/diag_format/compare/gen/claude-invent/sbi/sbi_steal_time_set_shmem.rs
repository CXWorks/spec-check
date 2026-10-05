pub open spec fn sbi_steal_time_set_shmem_spec(shmem_phys_lo: u64, shmem_phys_hi: u64, flags: u64, result: SbiRet, old_s: S, new_s: S) -> bool {
    let hart = CallingVirtualHart(old_s);
    let all_ones = shmem_phys_lo == 0xFFFF_FFFF_FFFF_FFFFu64 && shmem_phys_hi == 0xFFFF_FFFF_FFFF_FFFFu64;
    let base: int = (shmem_phys_hi as int) * 0x1_0000_0000_0000_0000int + (shmem_phys_lo as int);
    ((result.error == SBI_SUCCESS && !all_ones) ==> (
        flags == 0
        && (shmem_phys_lo as int) % 64 == 0
        && StealTimeReportingEnabled(new_s, hart)
        && StealTimeShmemBase(new_s, hart) == base
        && (forall|i: int| 0 <= i < 64 ==> PhysMemByte(new_s, base + i) == 0)
    ))
    && ((result.error == SBI_SUCCESS && all_ones) ==> (
        flags == 0
        && !StealTimeReportingEnabled(new_s, hart)
    ))
}
