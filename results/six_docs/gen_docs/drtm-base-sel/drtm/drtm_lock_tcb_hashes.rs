pub open spec fn drtm_lock_tcb_hashes_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == DRTM_SUCCESS ==> (new_s.drtm_tcb_hashes_locked == true))
    && (result == DRTM_DENIED ==> (new_s.drtm_tcb_hashes_locked == true))
    && (result == DRTM_NOT_SUPPORTED ==> (new_s.drtm_tcb_hashes_locked == old_s.drtm_tcb_hashes_locked))
    && (result != DRTM_SUCCESS && result != DRTM_DENIED && result != DRTM_NOT_SUPPORTED ==> (new_s.drtm_tcb_hashes_locked == old_s.drtm_tcb_hashes_locked))
}