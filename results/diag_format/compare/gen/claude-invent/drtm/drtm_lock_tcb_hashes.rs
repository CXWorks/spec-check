pub open spec fn drtm_lock_tcb_hashes_spec(result: DrtmReturnCode, old_s: S, new_s: S) -> bool {
    (!DrtmIsSupported(old_s) ==> result == NOT_SUPPORTED)
    && ((DrtmIsSupported(old_s) && TcbHashesLocked(old_s)) ==> (result == DENIED && TcbHashesLocked(new_s) == TcbHashesLocked(old_s)))
    && ((DrtmIsSupported(old_s) && !TcbHashesLocked(old_s)) ==> (result == SUCCESS && TcbHashesLocked(new_s)))
}
