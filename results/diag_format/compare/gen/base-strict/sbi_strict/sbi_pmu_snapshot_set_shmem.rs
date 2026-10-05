pub open spec fn sbi_pmu_snapshot_set_shmem_spec(result: ((), u64), old_s: S, new_s: S) -> bool {
    (
        (!(old_s.shmem_phys_lo == ALL_ONES && old_s.shmem_phys_hi == ALL_ONES) ==> PmuSnapshotShmemEnabled(new_s, CallingHart()))
        &&
        (!(old_s.shmem_phys_lo == ALL_ONES && old_s.shmem_phys_hi == ALL_ONES) ==> PmuSnapshotShmemBase(new_s, CallingHart()) == ConcatXlen(old_s.shmem_phys_hi, old_s.shmem_phys_lo))
        &&
        (!(old_s.shmem_phys_lo == ALL_ONES && old_s.shmem_phys_hi == ALL_ONES) ==> PmuSnapshotShmemSize(new_s, CallingHart()) == 4096)
        &&
        (!(old_s.shmem_phys_lo == ALL_ONES && old_s.shmem_phys_hi == ALL_ONES) ==> PmuSnapshotShmemLayoutMatchesTable45(new_s, CallingHart()))
        &&
        ((old_s.shmem_phys_lo == ALL_ONES && old_s.shmem_phys_hi == ALL_ONES) ==> !PmuSnapshotShmemEnabled(new_s, CallingHart()))
        &&
        ((old_s.shmem_phys_lo == ALL_ONES && old_s.shmem_phys_hi == ALL_ONES) ==> PmuSnapshotShmemCleared(new_s, CallingHart()))
    )
}