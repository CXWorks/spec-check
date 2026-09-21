pub open spec fn sbi_pmu_snapshot_set_shmem_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    let shmem_phys_lo = old_s.cmd_input_shmem_phys_lo;
    let shmem_phys_hi = old_s.cmd_input_shmem_phys_hi;
    let flags = old_s.cmd_input_flags;
    let is_all_ones = shmem_phys_lo == !0 && shmem_phys_hi == !0;
    let is_valid_addr = shmem_phys_lo & 0xFFF == 0;
    let is_clear_disable = is_all_ones;
    let is_set_enable = !is_all_ones;
    let is_success = result.is_Ok();
    let is_error = result.is_Err();
    (is_clear_disable ==> (is_success && new_s.pmu_snapshot_shmem_addr == 0 && new_s.pmu_snapshot_shmem_enabled == false))
    && (is_set_enable ==> (is_success ==> (is_valid_addr && new_s.pmu_snapshot_shmem_addr == ((shmem_phys_hi as u64) << 32 | shmem_phys_lo) && new_s.pmu_snapshot_shmem_enabled == true)))
    && (is_set_enable ==> (is_error ==> (is_valid_addr == false || new_s.pmu_snapshot_shmem_addr != ((shmem_phys_hi as u64) << 32 | shmem_phys_lo) || new_s.pmu_snapshot_shmem_enabled != true)))
    && (is_clear_disable ==> (is_error ==> false))
    && (is_set_enable ==> (is_error ==> false))
    && (is_success ==> (new_s.pmu_snapshot_shmem_addr == (if is_clear_disable then 0 else ((shmem_phys_hi as u64) << 32 | shmem_phys_lo)) && new_s.pmu_snapshot_shmem_enabled == (if is_clear_disable then false else true)))
}