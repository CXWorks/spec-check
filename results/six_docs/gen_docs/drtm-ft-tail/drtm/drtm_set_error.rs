pub open spec fn drtm_set_error_spec(error_code: Bits64, result: Result<(), DRTM_RETURN_CODE>, old_s: S, new_s: S) -> bool {
  (result == DRTM_SUCCESS ==> true)
  && (result == DRTM_NOT_SUPPORTED ==> true)
  && (result == DRTM_DENIED ==> true)
}