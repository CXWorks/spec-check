pub open spec fn drtm_get_error_spec(result: Int64, error_code: Int64, old_s: S, new_s: S) -> bool {
    (!IsDrtmSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsRecordedLaunchErrorCodeFound() ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> error_code == RecordedLaunchErrorCode())
    && (ResultEqual(result, SUCCESS) ==> old_s == new_s)
}