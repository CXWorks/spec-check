pub open spec fn drtm_lock_tcb_hashes_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!DrtmIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (TcbHashesLocked(old_s) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> TcbHashesLocked(new_s))
}