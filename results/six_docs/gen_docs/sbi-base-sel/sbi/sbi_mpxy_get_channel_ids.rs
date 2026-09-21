pub open spec fn sbi_mpxy_get_channel_ids_spec(result: int, start_index: UInt32, old_s: S, new_s: S) -> bool {
    (start_index < 0 ==> ResultEqual(result, SBI_SBI_ERR_INVALID_PARAM))
    && (old_s.shmem_enabled == false ==> ResultEqual(result, SBI_SBI_ERR_NO_SHMEM))
    && (old_s.hart_allowed_get_channel_ids == false ==> ResultEqual(result, SBI_SBI_ERR_DENIED))
    && (result == SBI_SBI_SUCCESS ==> (new_s.shmem_channel_ids_written == true))
    && (result != SBI_SBI_SUCCESS ==> (new_s.shmem_channel_ids_written == false))
}