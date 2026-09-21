pub open spec fn sbi_debug_set_shmem_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (flags(old_s) != 0 || (shmem_phys_lo(old_s) as int) % ((old_s.xlen as int) / 8) != 0))
    && (result == SBI_SBI_ERR_INVALID_ADDRESS ==> !shared_memory_requirements_satisfied(old_s, shmem_phys_lo(old_s), shmem_phys_hi(old_s)))
    && (result == SBI_SBI_ERR_FAILED ==> true)
    && (result == SBI_SBI_SUCCESS ==> (flags(old_s) == 0 && shared_memory_requirements_satisfied(old_s, shmem_phys_lo(old_s), shmem_phys_hi(old_s)) && shared_memory_state_changed(old_s, new_s)))
}

spec fn flags(s: S) -> int {
    s.flags
}

spec fn shmem_phys_lo(s: S) -> u64 {
    s.shmem_phys_lo
}

spec fn shmem_phys_hi(s: S) -> u64 {
    s.shmem_phys_hi
}

spec fn xlen(s: S) -> u64 {
    s.xlen
}

spec fn shared_memory_requirements_satisfied(s: S, lo: u64, hi: u64) -> bool {
    (lo == 0xFFFFFFFFFFFFFFFF && hi == 0xFFFFFFFFFFFFFFFF) || (lo % ((s.xlen as int) / 8) == 0)
}

spec fn shared_memory_state_changed(old_s: S, new_s: S) -> bool {
    (old_s.shmem_phys_lo == new_s.shmem_phys_lo) && (old_s.shmem_phys_hi == new_s.shmem_phys_hi) && (old_s.flags == new_s.flags)
}