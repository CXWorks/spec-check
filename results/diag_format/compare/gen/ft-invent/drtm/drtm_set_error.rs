pub open spec fn drtm_set_error_spec(error_code: Int64, result: Result<(), DRTMStatusCode>, old_s: S, new_s: S) -> bool {
  (result == DRTM_SUCCESS ==> DRTM_error_code(new_s) == error_code)
  && ((!(DRTM_error_code(old_s) == 0)) ==> result == DRTM_DENIED)
  && ((result != DRTM_SUCCESS)
    ==> DRTM_error_code(new_s) == DRTM_error_code(old_s))
}