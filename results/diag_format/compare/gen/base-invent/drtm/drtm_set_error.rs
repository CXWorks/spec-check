pub open spec fn drtm_set_error_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (result.is_Err() ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (result.is_Ok() ==> (old_s.drtm_error_code == 0 && new_s.drtm_error_code != 0))
}