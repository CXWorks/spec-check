pub open spec fn sdei_event_context_spec(param_id: UInt32, result: Result<int64, SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() && result.unwrap_err() == SDEI_ERROR_INVALID_PARAMETERS ==> (param_id > 17))
  && (result.is_Ok() ==> true)
}