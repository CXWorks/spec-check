pub open spec fn sdei_event_complete_and_resume_spec(resume_addr: Address, result: Result<(), SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (DispatcherDetectsInvalidResumeAddress(old_s, resume_addr) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (HandlerRunning(old_s, CallingPe()) == false ==> ResultEqual(result, DENIED))
  && (result == SDEI_SUCCESS ==> !ReturnsToCaller(new_s))
  && (result == SDEI_SUCCESS ==> HandlerRunning(new_s, CallingPe()) == false)
  && (result == SDEI_SUCCESS && IsPrivateEvent(new_s, RunningEvent(new_s, CallingPe())) ==> EventHandlingCompleteForPe(new_s, RunningEvent(new_s, CallingPe()), CallingPe()))
  && (result == SDEI_SUCCESS && IsSharedEvent(new_s, RunningEvent(new_s, CallingPe())) ==> EventHandlingCompleteGlobally(new_s, RunningEvent(new_s, CallingPe())))
  && (result == SDEI_SUCCESS ==> PeResumesAtElc(new_s, CallingPe(), resume_addr))
  && (result == SDEI_SUCCESS ==> ResumeContextMimicsSyncExceptionToElc(new_s, CallingPe(), EventTakenPc(new_s, CallingPe())))
  && (result == SDEI_SUCCESS && InterruptedYieldingSmc(new_s, CallingPe()) ==> ResumeHandlerRunsBeforeSecureExecution(new_s, CallingPe()))
  && ((SdeiIsSupported(old_s) &&
       !DispatcherDetectsInvalidResumeAddress(old_s, resume_addr) &&
       !(HandlerRunning(old_s, CallingPe()) == false))
    ==> result == SDEI_SUCCESS)
  && (result != SDEI_SUCCESS
    ==> HandlerRunning(new_s, CallingPe()) == HandlerRunning(old_s, CallingPe()))
  && (result != SDEI_SUCCESS
    ==> PeResumesAtElc(new_s, CallingPe(), resume_addr) == PeResumesAtElc(old_s, CallingPe(), resume_addr))
}