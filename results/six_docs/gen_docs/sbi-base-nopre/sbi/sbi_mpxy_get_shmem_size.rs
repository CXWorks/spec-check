pub open spec fn sbi_mpxy_get_shmem_size_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_SUCCESS)
    && (result.uvalue as int >= 4096)
    && ((result.uvalue as int) % 4096 == 0)
    && (result.uvalue as int >= MSG_DATA_MAX_LEN)
}