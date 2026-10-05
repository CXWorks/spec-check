pub open spec fn sdei_event_complete_spec(status_code: UInt32, result: Result<(), SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!HandlerRunning(old_s, CallingPe(old_s)) ==> ResultEqual(result, DENIED))
  && (result == SDEI_SUCCESS ==> HandlerRunning(new_s, CallingPe(new_s)) == false)
  && (result == SDEI_SUCCESS && IsPrivateEvent(old_s, event) ==> EventHandlingComplete(new_s, event, CallingPe(new_s)))
  && (result == SDEI_SUCCESS && IsSharedEvent(old_s, event) ==> EventHandlingCompleteGlobally(new_s, event))
  && ((SdeiIsSupported(old_s) &&
       HandlerRunning(old_s, CallingPe(old_s)))
    ==> result == SDEI_SUCCESS)
}