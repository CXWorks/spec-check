pub open spec fn sbi_debug_set_shmem_spec(result: i32, old_s: S, new_s: S) -> bool {
    (result == SBI_ERR_INVALID_PARAM ==> (flags != 0 || (shmem_phys_lo as int) % ((XLEN as int) / 8) != 0))
    && (result == SBI_ERR_INVALID_ADDRESS ==> !valid_shared_memory_address(shmem_phys_lo, shmem_phys_hi))
    && (result == SBI_ERR_FAILED ==> true)
    && (result == SBI_SUCCESS ==> (flags == 0 && (shmem_phys_lo as int) % ((XLEN as int) / 8) == 0 && valid_shared_memory_address(shmem_phys_lo, shmem_phys_hi)))
}

fn flags(old_s: S) -> u64 {
    old_s.cmd_input_flags
}

fn shmem_phys_lo(old_s: S) -> u64 {
    old_s.cmd_input_shmem_phys_lo
}

fn shmem_phys_hi(old_s: S) -> u64 {
    old_s.cmd_input_shmem_phys_hi
}

fn valid_shared_memory_address(shmem_phys_lo: u64, shmem_phys_hi: u64) -> bool {
    shmem_phys_lo != -1 && shmem_phys_hi != -1
}