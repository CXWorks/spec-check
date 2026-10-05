pub open spec fn drtm_set_error_spec(error_code: Bits64, result: Int64, old_s: S, new_s: S) -> bool {
    (!DrtmIsSupported(old_s) ==> result == NOT_SUPPORTED)
    && ((DrtmIsSupported(old_s) && DrtmErrorIsSet(old_s)) ==> (result == DENIED && DrtmStoredError(new_s) == DrtmStoredError(old_s)))
    && ((DrtmIsSupported(old_s) && !DrtmErrorIsSet(old_s)) ==> (
        result == SUCCESS
        && DrtmErrorIsSet(new_s)
        && (!DrtmDcePhaseCompleted(old_s) ==> DrtmStoredError(new_s) == error_code)
        && (DrtmDcePhaseCompleted(old_s) ==> (
            (DrtmStoredError(new_s) & 0xFFFF_FFFF_FFFF_F800u64) == (error_code & 0xFFFF_FFFF_FFFF_F800u64)
            && (DrtmStoredError(new_s) & 0x7u64) == DrtmPhaseDlme()
            && ((DrtmStoredError(new_s) >> 3u64) & 0xFFu64) == 0xFFu64
        ))
    ))
}
