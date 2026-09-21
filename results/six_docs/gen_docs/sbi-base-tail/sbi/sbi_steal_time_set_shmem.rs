pub open spec fn sbi_steal_time_set_shmem_spec(result: int, old_s: S, new_s: S) -> bool {
    let shmem_phys_lo: u64 = old_s.cmd_input_shmem_phys_lo;
    let shmem_phys_hi: u64 = old_s.cmd_input_shmem_phys_hi;
    let flags: u64 = old_s.cmd_input_flags;
    let all_ones: u64 = !0;
    let is_all_ones: bool = (shmem_phys_lo == all_ones) && (shmem_phys_hi == all_ones);
    let is_aligned: bool = (shmem_phys_lo % 64) == 0;
    let flags_is_zero: bool = flags == 0;
    (!is_all_ones ==> (is_aligned && flags_is_zero && result == SBI_SBI_ERR_INVALID_PARAM))
    && (is_all_ones ==> result == SBI_SBI_SUCCESS)
    && (flags_is_zero ==> true)
    && (is_aligned ==> true)
    && (result == SBI_SBI_SUCCESS ==> (new_s.cmd_output_shmem_base == (shmem_phys_hi << 32) | shmem_phys_lo))
    && (result == SBI_SBI_SUCCESS ==> (new_s.cmd_output_shmem_size >= 64))
    && (result == SBI_SBI_SUCCESS ==> (new_s.cmd_output_shmem_first_64_bytes == 0))
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> (new_s.cmd_output_shmem_base == old_s.cmd_output_shmem_base))
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> (new_s.cmd_output_shmem_size == old_s.cmd_output_shmem_size))
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> (new_s.cmd_output_shmem_first_64_bytes == old_s.cmd_output_shmem_first_64_bytes))
}