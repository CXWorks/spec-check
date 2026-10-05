pub open spec fn sdei_event_complete_and_resume_spec(resume_addr: Address, result: Int64, old_s: S, new_s: S) -> bool {
  (!IsSdeiSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (IsIdentifiablyInvalidResumeAddress(old_s, resume_addr) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (HandlerRunning(old_s, CallingPe(old_s)) == false ==> ResultEqual(result, DENIED))
  && (result == SDEI_SUCCESS ==> ResumeContextExceptionReturnAddress(new_s) == InterruptedPc(new_s))
  && (result == SDEI_SUCCESS ==> HandlerRunning(new_s, CallingPe(new_s)) == false)
  && (result == SDEI_SUCCESS ==> IsPrivateEvent(old_s, CurrentEvent(old_s)) ==> EventHandlingComplete(new_s, CurrentEvent(new_s), CallingPe(new_s)))
  && (result == SDEI_SUCCESS ==> IsSharedEvent(old_s, CurrentEvent(old_s)) ==> EventHandlingCompleteGlobally(new_s, CurrentEvent(new_s)))
  && ((IsSdeiSupported(old_s) &&
       !IsIdentifiablyInvalidResumeAddress(old_s, resume_addr) &&
       !(HandlerRunning(old_s, CallingPe(old_s)) == false))
    ==> result == SDEI_SUCCESS)
  && (result != SDEI_SUCCESS
    ==> ResumeContextExceptionReturnAddress(new_s) == ResumeContextExceptionReturnAddress(old_s))
  && (result != SDEI_SUCCESS
    ==> HandlerRunning(new_s, CallingPe(new_s)) == HandlerRunning(old_s, CallingPe(old_s)))
  && (result != SDEI_SUCCESS
    ==> EventHandlingComplete(new_s, CurrentEvent(new_s), CallingPe(new_s)) == EventHandlingComplete(old_s, CurrentEvent(old_s), CallingPe(old_s)))
  && (result != SDEI_SUCCESS
    ==> EventHandlingCompleteGlobally(new_s, CurrentEvent(new_s)) == EventHandlingCompleteGlobally(old_s, CurrentEvent(old_s)))
}