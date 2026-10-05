pub open spec fn sbi_steal_time_set_shmem_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    let shmem_phys_lo: u64 = old_s.cmd_input_shmem_phys_lo;
    let shmem_phys_hi: u64 = old_s.cmd_input_shmem_phys_hi;
    let flags: u64 = old_s.cmd_input_flags;
    let is_all_ones: bool = shmem_phys_lo == !0 && shmem_phys_hi == !0;
    let is_aligned: bool = (shmem_phys_lo as int) % 64 == 0;
    let flags_is_zero: bool = flags == 0;
    (!is_all_ones ==> (is_aligned && result.is_Ok()))
    && (is_all_ones ==> result.is_Ok())
    && (flags_is_zero ==> result.is_Ok())
    && (!flags_is_zero ==> ResultEqual(result, SBI_ERROR_INVALID_PARAM))
    && (result.is_Ok() ==> (new_s.steal_time_shmem_base == (shmem_phys_hi as u64) << 64 | shmem_phys_lo))
    && (result.is_Ok() ==> (new_s.steal_time_shmem_enabled == !is_all_ones))
    && (result.is_Ok() ==> (new_s.steal_time_shmem_base == (new_s.steal_time_shmem_base as int) as u64))
}