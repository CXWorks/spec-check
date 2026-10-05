pub open spec fn drtm_lock_tcb_hashes_spec(result: Result<(), NotSupported>, old_s: S, new_s: S) -> bool {
  (!DrtmIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (TcbHashesLocked(old_s) ==> ResultEqual(result, DENIED))
  && (result.is_Ok() ==> ResultEqual(result, SUCCESS))
  && (result.is_Ok() ==> TcbHashesLocked(new_s))
  && ((DrtmIsSupported(old_s) &&
       !TcbHashesLocked(old_s))
    ==> result.is_Ok())
}