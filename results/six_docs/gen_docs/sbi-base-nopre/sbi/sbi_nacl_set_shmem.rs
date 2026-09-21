pub open spec fn sbi_nacl_set_shmem_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> (flags != 0 || (shmem_phys_lo as int) % 4096 != 0))
    && (result.error == SBI_ERR_INVALID_ADDRESS ==> !valid_shared_memory_address(shmem_phys_lo, shmem_phys_hi))
    && (result.error == SBI_ERR_FAILED ==> true)
    && (result.error == SBI_SUCCESS ==> (flags == 0 && ((shmem_phys_lo == 0xFFFFFFFFFFFFFFFF && shmem_phys_hi == 0xFFFFFFFFFFFFFFFF) || ((shmem_phys_lo as int) % 4096 == 0 && (shmem_phys_lo + 4096 + (XLEN * 128)) <= (1u64 << XLEN)))))
    && (result.error == SBI_SUCCESS ==> new_s.nacl_shmem_base == (shmem_phys_hi << XLEN | shmem_phys_lo))
    && (result.error == SBI_ERR_INVALID_PARAM || result.error == SBI_ERR_INVALID_ADDRESS || result.error == SBI_ERR_FAILED ==> new_s.nacl_shmem_base == old_s.nacl_shmem_base)
}

fn valid_shared_memory_address(shmem_phys_lo: UInt64, shmem_phys_hi: UInt64) -> bool {
    let base = (shmem_phys_hi << XLEN) | shmem_phys_lo;
    let size = 4096 + (XLEN * 128);
    base + size <= (1u64 << XLEN)
}