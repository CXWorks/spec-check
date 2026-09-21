pub open spec fn sbi_mpxy_get_channel_ids_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> <unconstrained>)
    && (result == SBI_SBI_ERR_NO_SHMEM ==> <unconstrained>)
    && (result == SBI_SBI_ERR_DENIED ==> <unconstrained>)
    && (result == SBI_SBI_ERR_FAILED ==> <unconstrained>)
    && (result == SBI_SBI_SUCCESS ==> <unconstrained>)
}