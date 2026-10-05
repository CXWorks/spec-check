pub open spec fn drtm_lock_tcb_hashes_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> DRTM_TCB_HASHES_LOCKED(new_s))
  && (result == RSI_ERROR_DENIED ==> DRTM_TCB_HASHES_LOCKED(old_s))
  && ((!(DRTM_TCB_HASHES_LOCKED(old_s)))
    ==> result == RSI_SUCCESS)
}