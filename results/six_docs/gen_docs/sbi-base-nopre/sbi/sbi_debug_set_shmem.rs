pub open spec fn sbi_debug_set_shmem_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> (flags != 0 || (shmem_phys_lo as int) % ((XLEN as int) / 8) != 0))
    && (result.error == SBI_ERR_INVALID_ADDRESS ==> !valid_shared_memory(shmem_phys_lo, shmem_phys_hi))
    && (result.error == SBI_ERR_FAILED ==> true)
    && (result.error == SBI_SUCCESS ==> (flags == 0 && (shmem_phys_lo as int) % ((XLEN as int) / 8) == 0 && valid_shared_memory(shmem_phys_lo, shmem_phys_hi)))
}

fn flags(s: S) -> u64 {
    s.cmd_input_flags
}

fn shmem_phys_lo(s: S) -> u64 {
    s.cmd_input_shmem_phys_lo
}

fn shmem_phys_hi(s: S) -> u64 {
    s.cmd_input_shmem_phys_hi
}

fn valid_shared_memory(lo: u64, hi: u64) -> bool {
    (lo == !0 || (hi == !0 && lo == !0)) || (lo as int) >= 0
}