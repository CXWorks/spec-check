pub open spec fn sdei_event_context_spec(param_id: UInt32, result: Result<int64, SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (result == SDEI_ERROR_INVALID_PARAMETERS ==> result == SDEI_ERROR_INVALID_PARAMETERS)
  && (result == SDEI_ERROR_DENIED ==> result == SDEI_ERROR_DENIED)
  && (result == SDEI_ERROR_NOT_SUPPORTED ==> result == SDEI_ERROR_NOT_SUPPORTED)
  && ((!(SdeiEventHandlerAt(old_s, 0).handler_running)) ==> result == SDEI_ERROR_DENIED)
  && (result == SDEI_SUCCESS && (param_id > 17) ==> result == SDEI_ERROR_INVALID_PARAMETERS)
  && ((result != SDEI_SUCCESS && result != SDEI_ERROR_INVALID_PARAMETERS && result != SDEI_ERROR_DENIED && result != SDEI_ERROR_NOT_SUPPORTED)
    ==> result == SDEI_SUCCESS)
}