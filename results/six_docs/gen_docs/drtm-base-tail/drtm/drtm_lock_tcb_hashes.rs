pub open spec fn drtm_lock_tcb_hashes_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == DRTM_NOT_SUPPORTED ==> true)
    && (result == DRTM_DENIED ==> true)
    && (result == DRTM_SUCCESS ==> true)
}