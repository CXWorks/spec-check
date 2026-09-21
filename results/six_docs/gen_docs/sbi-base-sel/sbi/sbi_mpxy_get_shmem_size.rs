pub open spec fn sbi_mpxy_get_shmem_size_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_SUCCESS)
    && (old_s == new_s)
    && (result as int >= 4096)
    && (result as int) % 4096 == 0
}