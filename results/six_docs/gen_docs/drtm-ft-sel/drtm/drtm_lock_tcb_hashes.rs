pub open spec fn drtm_lock_tcb_hashes_spec(result: int64, old_s: S, new_s: S) -> bool {
  (result == DRTM_DENIED ==> true)
  && ((!(result == DRTM_DENIED))
    ==> result == DRTM_SUCCESS)
}