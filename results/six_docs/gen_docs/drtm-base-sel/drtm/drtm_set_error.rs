pub open spec fn drtm_set_error_spec(result: int, old_s: S, new_s: S) -> bool {
    (old_s.drtm_set_error_called ==> ResultEqual(result, DRTM_DENIED))
    && (!old_s.drtm_set_error_called ==> (result == DRTM_SUCCESS))
    && (old_s.drtm_set_error_called ==> new_s.drtm_set_error_called)
    && (!old_s.drtm_set_error_called ==> new_s.drtm_set_error_called)
}