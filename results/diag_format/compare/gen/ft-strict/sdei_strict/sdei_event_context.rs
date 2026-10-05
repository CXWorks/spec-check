pub open spec fn sdei_event_context_spec(param_id: UInt32, result: Result<Int64, SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (param_id > 17 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!HandlerRunning(old_s, CallingPe()) ==> ResultEqual(result, DENIED))
  && (result.is_Ok() ==> ResultEqual(result, EventContextRegister(new_s, CallingPe(), param_id as int)))
  && ((SdeiIsSupported(old_s) &&
       !(param_id > 17) &&
       HandlerRunning(old_s, CallingPe()))
    ==> result.is_Ok())
}