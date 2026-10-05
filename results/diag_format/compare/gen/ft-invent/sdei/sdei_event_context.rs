pub open spec fn sdei_event_context_spec(param_id: UInt32, result: Result<int64, SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (result == SDEI_ERROR_INVALID_PARAMETERS ==> result.is_Err())
  && (result == SDEI_ERROR_DENIED ==> result.is_Err())
  && ((!(SdeiEventHandlerAt(old_s, 0).handler_running)) ==> result == SDEI_ERROR_DENIED)
  && (result.is_Ok() ==> true)
  && (result == SDEI_SUCCESS ==> true)
}