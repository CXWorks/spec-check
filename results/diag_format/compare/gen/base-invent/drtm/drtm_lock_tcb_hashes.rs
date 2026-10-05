pub open spec fn drtm_lock_tcb_hashes_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (RsiCommandReturnCode::RSI_ERROR_STATE == RsiCommandReturnCode::DENIED ==> (new_s.drtm_tcb_hashes_locked == true))
    && (RsiCommandReturnCode::RSI_SUCCESS == result ==> (new_s.drtm_tcb_hashes_locked == true))
    && (RsiCommandReturnCode::RSI_ERROR_STATE != result ==> (new_s.drtm_tcb_hashes_locked == old_s.drtm_tcb_hashes_locked))
}