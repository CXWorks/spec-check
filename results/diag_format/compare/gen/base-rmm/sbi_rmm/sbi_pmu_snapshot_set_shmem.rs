pub open spec fn sbi_pmu_snapshot_set_shmem_spec(result: i64, value: i64, old_s: S, new_s: S) -> bool {
    let shmem_phys_lo = old_s.cmd_input_shmem_phys_lo;
    let shmem_phys_hi = old_s.cmd_input_shmem_phys_hi;
    let is_clear = IsAllOnes(shmem_phys_lo) && IsAllOnes(shmem_phys_hi);
    let is_set = !is_clear;
    let expected_base = ((shmem_phys_hi as u64) << 64) | (shmem_phys_lo as u64);
    let pmu_shmem = PmuSnapshotShmem(CurrentHart());
    let new_base = new_s.PmuSnapshotShmem(CurrentHart()).base;
    let new_size = new_s.PmuSnapshotShmem(CurrentHart()).size;
    let new_enabled = new_s.PmuSnapshotShmem(CurrentHart()).enabled;
    let new_cleared = new_s.PmuSnapshotShmem(CurrentHart()).cleared;
    let old_base = old_s.PmuSnapshotShmem(CurrentHart()).base;
    let old_size = old_s.PmuSnapshotShmem(CurrentHart()).size;
    let old_enabled = old_s.PmuSnapshotShmem(CurrentHart()).enabled;
    let old_cleared = old_s.PmuSnapshotShmem(CurrentHart()).cleared;
    let addr_aligned = AddrIsPageAligned(shmem_phys_lo);
    (is_set ==> (addr_aligned && result == 0 && value == 0 && new_base == expected_base && new_size == 4096 && new_enabled == true && new_cleared == false))
    && (is_clear ==> (result == 0 && value == 0 && new_base == 0 && new_size == 0 && new_enabled == false && new_cleared == true))
    && (is_set ==> (old_base == old_base && old_size == old_size && old_enabled == old_enabled && old_cleared == old_cleared))
    && (is_clear ==> (old_base == old_base && old_size == old_size && old_enabled == old_enabled && old_cleared == old_cleared))
}