pub open spec fn sdei_event_complete_and_resume_spec(resume_addr: UInt64, result: Result<(), SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (result == SDEI_ERROR_INVALID_PARAMETERS ==> (resume_addr % 4 != 0))
  && (result == SDEI_ERROR_DENIED ==> HandlerRunning(old_s, current_pe()) == false)
  && ((!(resume_addr % 4 != 0) &&
       HandlerRunning(old_s, current_pe()) == true)
    ==> result == SDEI_SUCCESS)
  && (result == SDEI_SUCCESS
    ==> HandlerRunning(new_s, current_pe()) == false)
  && ((!(result == SDEI_ERROR_INVALID_PARAMETERS) &&
       !(result == SDEI_ERROR_DENIED))
    ==> result == SDEI_SUCCESS)
  && (result != SDEI_SUCCESS
    ==> HandlerRunning(new_s, current_pe()) == HandlerRunning(old_s, current_pe()))
}