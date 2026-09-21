pub open spec fn drtm_set_error_spec(error_code: Int64, result: Result<(), DRTMSetErrorStatusCode>, old_s: S, new_s: S) -> bool {
  (result == DRTM_SET_ERROR_SUCCESS ==> true)
  && (result == DRTM_SET_ERROR_NOT_SUPPORTED ==> true)
  && (result == DRTM_SET_ERROR_DENIED ==> true)
  && ((!(result == DRTM_SET_ERROR_SUCCESS) &&
       !(result == DRTM_SET_ERROR_NOT_SUPPORTED) &&
       !(result == DRTM_SET_ERROR_DENIED))
    ==> true)
}