pub open spec fn drtm_get_error_spec(result: i64, x1: i64, old_s: S, new_s: S) -> bool {
    (!DrtmIsSupported(old_s) ==> result == NOT_SUPPORTED)
    && ((DrtmIsSupported(old_s) && !DrtmErrorCodeFound(old_s)) ==> result == NOT_FOUND)
    && ((DrtmIsSupported(old_s) && DrtmErrorCodeFound(old_s)) ==> (
        result == SUCCESS
        && (DrtmPreviousLaunchOccurred(old_s) ==> x1 == DrtmRecordedErrorCode(old_s))
    ))
    && (DrtmRecordedErrorCode(new_s) == DrtmRecordedErrorCode(old_s))
}
