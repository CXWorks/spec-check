pub open spec fn sbi_mpxy_get_channel_ids_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> (result.uvalue == 0 && (new_s.shmem_start_index as int) < 0 || (new_s.shmem_start_index as int) >= (old_s.shmem_channel_count as int)))
    && (result.error == SBI_ERR_NO_SHMEM ==> (result.uvalue == 0 && !old_s.shmem_enabled))
    && (result.error == SBI_ERR_DENIED ==> (result.uvalue == 0 && !old_s.shmem_accessible))
    && (result.error == SBI_ERR_FAILED ==> (result.uvalue == 0))
    && (result.error == SBI_SUCCESS ==> (result.uvalue == 0 && (new_s.shmem_returned as int) > 0 && (new_s.shmem_remaining as int) >= 0 && (new_s.shmem_start_index as int) >= 0 && (new_s.shmem_start_index as int) < (old_s.shmem_channel_count as int) && (new_s.shmem_returned as int) <= (old_s.shmem_channel_count as int) - (new_s.shmem_start_index as int)))
}