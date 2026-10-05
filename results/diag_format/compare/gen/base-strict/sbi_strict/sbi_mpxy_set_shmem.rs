pub open spec fn sbi_mpxy_set_shmem_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (!((old_s.shmem_phys_lo == -1) && (old_s.shmem_phys_hi == -1)) ==> (new_s.SharedMemoryBase(CallingHart()) == ConcatPhysAddr(old_s.shmem_phys_hi, old_s.shmem_phys_lo)))
    && (!((old_s.shmem_phys_lo == -1) && (old_s.shmem_phys_hi == -1)) ==> (new_s.SharedMemorySize(CallingHart()) == MpxyGetShmemSize()))
    && (((old_s.shmem_phys_lo == -1) && (old_s.shmem_phys_hi == -1)) ==> new_s.SharedMemoryDisabled(CallingHart()))
    && (new_s.SharedMemorySetupMode(CallingHart()) == Bits(old_s.flags, 1, 0))
    && (result == sbiret { error: 0, value: () })
}