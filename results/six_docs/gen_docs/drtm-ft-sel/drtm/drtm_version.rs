pub open spec fn drtm_version_spec(result: DrtmStatusCode, old_s: S, new_s: S) -> bool {
  (result == DRTM_SUCCESS ==> (result == DRTM_SUCCESS))
}