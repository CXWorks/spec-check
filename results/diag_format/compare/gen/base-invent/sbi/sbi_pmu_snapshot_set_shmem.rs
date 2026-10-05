pub open spec fn sbi_pmu_snapshot_set_shmem_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    let shmem_phys_lo = old_s.cmd_input_shmem_phys_lo;
    let shmem_phys_hi = old_s.cmd_input_shmem_phys_hi;
    let is_all_ones = shmem_phys_lo == !0 && shmem_phys_hi == !0;
    let is_aligned = shmem_phys_lo % 4096 == 0;
    let is_valid_addr = !is_all_ones;
    let is_valid_size = !is_all_ones;
    (!is_aligned ==> result.code != 0)
    && (!is_valid_addr ==> result.code != 0)
    && (!is_valid_size ==> result.code != 0)
    && (is_all_ones ==> result.code == 0)
    && (!is_all_ones && is_aligned && is_valid_addr && is_valid_size ==> result.code == 0)
    && (is_all_ones ==> new_s.pmu_snapshot_shmem_addr == 0)
    && (!is_all_ones ==> new_s.pmu_snapshot_shmem_addr == (shmem_phys_hi as u64) << 32 | (shmem_phys_lo as u64))
    && (is_all_ones ==> new_s.pmu_snapshot_shmem_enabled == false)
    && (!is_all_ones ==> new_s.pmu_snapshot_shmem_enabled == true)
    && (is_all_ones ==> new_s.pmu_snapshot_shmem_size == 0)
    && (!is_all_ones ==> new_s.pmu_snapshot_shmem_size == 4096)
}