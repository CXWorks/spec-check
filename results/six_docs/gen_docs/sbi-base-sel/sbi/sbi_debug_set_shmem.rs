pub open spec fn sbi_debug_set_shmem_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (flags(old_s) != 0 || (shmem_phys_lo(old_s) as int) % ((old_s.xlen as int) / 8) != 0))
    && (result == SBI_SBI_ERR_INVALID_ADDRESS ==> !valid_shared_memory(old_s, shmem_phys_lo(old_s), shmem_phys_hi(old_s)))
    && (result == SBI_SBI_ERR_FAILED ==> true)
    && (result == SBI_SBI_SUCCESS ==> (flags(old_s) == 0 && (shmem_phys_lo(old_s) as int) % ((old_s.xlen as int) / 8) == 0 && valid_shared_memory(old_s, shmem_phys_lo(old_s), shmem_phys_hi(old_s))))
}

spec fn flags(s: S) -> int {
    s.debug_trigger_shmem_flags
}

spec fn shmem_phys_lo(s: S) -> UInt64 {
    s.debug_trigger_shmem_phys_lo
}

spec fn shmem_phys_hi(s: S) -> UInt64 {
    s.debug_trigger_shmem_phys_hi
}

spec fn valid_shared_memory(s: S, lo: UInt64, hi: UInt64) -> bool {
    lo == 0xFFFFFFFFFFFFFFFF && hi == 0xFFFFFFFFFFFFFFFF || (lo != 0xFFFFFFFFFFFFFFFF && hi != 0xFFFFFFFFFFFFFFFF)
}