pub open spec fn drtm_get_error_spec(result: int64, old_s: S, new_s: S) -> bool {
    (old_s.drtm_error_code == 0 ==> result == 0)
    && (old_s.drtm_error_code != 0 ==> result == old_s.drtm_error_code)
    && (result == 0 ==> old_s.drtm_error_code == 0)
    && (result != 0 ==> old_s.drtm_error_code == result)
}