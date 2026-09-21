pub open spec fn drtm_lock_tcb_hashes_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> <unconstrained>)
    && (result == RSI_SUCCESS ==> (old_s.drtm_tcb_hashes_locked == true && new_s.drtm_tcb_hashes_locked == true))
    && (result == RSI_ERROR_STATE ==> (old_s.drtm_tcb_hashes_locked == false && new_s.drtm_tcb_hashes_locked == true))
    && (result == RSI_ERROR_UNKNOWN ==> <unconstrained>)
}