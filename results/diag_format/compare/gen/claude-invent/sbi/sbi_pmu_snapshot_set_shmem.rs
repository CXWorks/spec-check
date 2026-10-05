pub open spec fn sbi_pmu_snapshot_set_shmem_spec(result: SbiRet, old_s: S, new_s: S, shmem_phys_lo: UInt64, shmem_phys_hi: UInt64, flags: UInt64) -> bool {
    (SbiRetIsSuccess(result) && shmem_phys_lo == 0xFFFF_FFFF_FFFF_FFFFu64 && shmem_phys_hi == 0xFFFF_FFFF_FFFF_FFFFu64
        ==> !PmuSnapshotEnabled(new_s, CallingHart(old_s)))
    && (!(shmem_phys_lo == 0xFFFF_FFFF_FFFF_FFFFu64 && shmem_phys_hi == 0xFFFF_FFFF_FFFF_FFFFu64) && (shmem_phys_lo as int) % 4096 != 0
        ==> !SbiRetIsSuccess(result))
    && (SbiRetIsSuccess(result) && !(shmem_phys_lo == 0xFFFF_FFFF_FFFF_FFFFu64 && shmem_phys_hi == 0xFFFF_FFFF_FFFF_FFFFu64)
        ==> (shmem_phys_lo as int) % 4096 == 0
            && PmuSnapshotEnabled(new_s, CallingHart(old_s))
            && PmuSnapshotShmemBase(new_s, CallingHart(old_s)) == PhysAddrFromParts(shmem_phys_lo, shmem_phys_hi)
            && PmuSnapshotShmemSize(new_s, CallingHart(old_s)) == 4096)
    && (!SbiRetIsSuccess(result)
        ==> PmuSnapshotEnabled(new_s, CallingHart(old_s)) == PmuSnapshotEnabled(old_s, CallingHart(old_s))
            && PmuSnapshotShmemBase(new_s, CallingHart(old_s)) == PmuSnapshotShmemBase(old_s, CallingHart(old_s))
            && PmuSnapshotShmemSize(new_s, CallingHart(old_s)) == PmuSnapshotShmemSize(old_s, CallingHart(old_s)))
    && (forall|h: UInt64| h != CallingHart(old_s) ==>
            PmuSnapshotEnabled(new_s, h) == PmuSnapshotEnabled(old_s, h)
            && PmuSnapshotShmemBase(new_s, h) == PmuSnapshotShmemBase(old_s, h)
            && PmuSnapshotShmemSize(new_s, h) == PmuSnapshotShmemSize(old_s, h))
}
