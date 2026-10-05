pub open spec fn sdei_event_complete_spec(status_code: UInt32, result: Result<(), SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!HandlerRunning(old_s, CallingPe(old_s)) ==> ResultEqual(result, DENIED))
  && (result == SDEI_SUCCESS ==> !CallReturns(new_s, CallingPe(new_s)))
  && (result == SDEI_SUCCESS ==> ExecutionResumesAtInterruptedContext(new_s, CallingPe(new_s)))
  && (result == SDEI_SUCCESS ==> !HandlerRunning(new_s, CallingPe(new_s)))
  && (result == SDEI_SUCCESS && IsPrivateEvent(new_s, RunningEvent(new_s, CallingPe(new_s))) ==> EventHandlingCompleteForPe(new_s, RunningEvent(new_s, CallingPe(new_s)), CallingPe(new_s)))
  && (result == SDEI_SUCCESS && IsSharedEvent(new_s, RunningEvent(new_s, CallingPe(new_s))) ==> EventHandlingCompleteGlobally(new_s, RunningEvent(new_s, CallingPe(new_s))))
  && ((SdeiIsSupported(old_s) &&
       HandlerRunning(old_s, CallingPe(old_s)))
    ==> result == SDEI_SUCCESS)
  && (result != SDEI_SUCCESS
    ==> CallReturns(new_s, CallingPe(new_s)))
  && (result != SDEI_SUCCESS
    ==> !ExecutionResumesAtInterruptedContext(new_s, CallingPe(new_s)))
  && (result != SDEI_SUCCESS
    ==> HandlerRunning(new_s, CallingPe(new_s)))
  && (result != SDEI_SUCCESS && !(IsPrivateEvent(new_s, RunningEvent(new_s, CallingPe(new_s))))
    ==> !EventHandlingCompleteForPe(new_s, RunningEvent(new_s, CallingPe(new_s)), CallingPe(new_s)))
  && (result != SDEI_SUCCESS && !(IsSharedEvent(new_s, RunningEvent(new_s, CallingPe(new_s))))
    ==> !EventHandlingCompleteGlobally(new_s, RunningEvent(new_s, CallingPe(new_s))))
}