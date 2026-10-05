pub open spec fn sdei_event_context_spec(param_id: UInt32, result: Result<(), SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (!IsSdeiSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidContextParamId(old_s, param_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsHandlerRunningOnPe(old_s, CurrentPe()) ==> ResultEqual(result, DENIED))
  && (result == SDEI_SUCCESS ==> result == EventContextRegister(new_s, CurrentPe(), param_id))
  && ((IsSdeiSupported(old_s) &&
       IsValidContextParamId(old_s, param_id) &&
       IsHandlerRunningOnPe(old_s, CurrentPe()))
    ==> result == SDEI_SUCCESS)
  && (result != SDEI_SUCCESS
    ==> EventContextRegister(new_s, CurrentPe(), param_id) == EventContextRegister(old_s, CurrentPe(), param_id))
}