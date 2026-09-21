pub open spec fn drtm_set_error_spec(error_code: Int64, result: Result<(), DrtmStatusCode>, old_s: S, new_s: S) -> bool {
  (result == DRTM_NOT_SUPPORTED ==> result == DRTM_NOT_SUPPORTED)
}