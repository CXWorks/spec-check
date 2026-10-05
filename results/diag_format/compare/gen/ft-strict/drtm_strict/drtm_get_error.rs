pub open spec fn drtm_get_error_spec(result: Result<(), DrtmGetErrorReturnCode>, error_code: Int64, old_s: S, new_s: S) -> bool {
  (!IsDrtmSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsRecordedLaunchErrorCodeFound(old_s) ==> ResultEqual(result, NOT_FOUND))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> error_code == RecordedLaunchErrorCode(new_s))
  && (result == SUCCESS ==> RecordedLaunchErrorCodeUnchanged(new_s))
  && ((IsDrtmSupported(old_s) &&
       IsRecordedLaunchErrorCodeFound(old_s))
    ==> result == SUCCESS)
}