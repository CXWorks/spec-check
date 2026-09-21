pub open spec fn sbi_steal_time_set_shmem_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    let shmem_phys_lo: u64 = old_s.cmd_input_shmem_phys_lo;
    let shmem_phys_hi: u64 = old_s.cmd_input_shmem_phys_hi;
    let flags: u64 = old_s.cmd_input_flags;
    let is_all_ones: bool = shmem_phys_lo == !0 && shmem_phys_hi == !0;
    let is_valid_addr: bool = !is_all_ones;
    let is_aligned: bool = (shmem_phys_lo as int) % 64 == 0;
    let is_flags_zero: bool = flags == 0;
    let is_success: bool = result == 0;
    (is_all_ones ==> result == 0)
    && (!is_all_ones ==> (is_valid_addr && is_aligned && is_flags_zero ==> result == 0))
    && (is_success ==> (new_s.steal_time_shmem_base == (shmem_phys_hi as u64) << 64 | shmem_phys_lo))
}