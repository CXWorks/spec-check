pub open spec fn drtm_set_error_spec(result: Result<(), RsiStatusCode>, old_s: S, new_s: S) -> bool {
    (result.is_Err() ==> result == RSI_ERROR_INPUT)
    && (result.is_Ok() ==> (old_s.drtm_set_error_called == false && new_s.drtm_set_error_called == true))
}