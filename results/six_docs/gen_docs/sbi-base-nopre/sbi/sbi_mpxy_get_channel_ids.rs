pub open spec fn sbi_mpxy_get_channel_ids_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> (result.uvalue == 0 && (start_index as int) < 0))
    && (result.error == SBI_ERR_NO_SHMEM ==> (result.uvalue == 0))
    && (result.error == SBI_ERR_DENIED ==> (result.uvalue == 0))
    && (result.error == SBI_ERR_FAILED ==> (result.uvalue == 0))
    && (result.error == SBI_SUCCESS ==> (result.uvalue == 0))
}