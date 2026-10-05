pub open spec fn sbi_nacl_set_shmem_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> (flags != 0 || (shmem_phys_lo as int) % 4096 != 0))
    && (result.error == SBI_ERR_INVALID_ADDRESS ==> !valid_shared_memory_address(old_s, shmem_phys_lo, shmem_phys_hi))
    && (result.error == SBI_ERR_FAILED ==> true)
    && (result.error == SBI_SUCCESS ==> (flags == 0 && ((shmem_phys_lo == 0xFFFFFFFFFFFFFFFF && shmem_phys_hi == 0xFFFFFFFFFFFFFFFF) || ((shmem_phys_lo as int) % 4096 == 0) && valid_shared_memory_address(old_s, shmem_phys_lo, shmem_phys_hi))))
}

fn valid_shared_memory_address(s: S, shmem_phys_lo: UInt64, shmem_phys_hi: UInt64) -> bool {
    let base_addr = if shmem_phys_lo == 0xFFFFFFFFFFFFFFFF && shmem_phys_hi == 0xFFFFFFFFFFFFFFFF {
        0
    } else {
        (shmem_phys_hi as UInt64) << 64 | shmem_phys_lo
    };
    let size = 4096 + (128 * (s.xlen as UInt64));
    base_addr + size <= s.max_physical_address
}