pub open spec fn sdei_event_complete_and_resume_spec(resume_addr: UInt64, result: int64, old_s: S, new_s: S) -> bool {
  (result == SDEI_INVALID_PARAMETERS ==> (resume_addr % 4 != 0))
  && (result == SDEI_DENIED ==> (handler_running(old_s) == false))
  && ((!(result == SDEI_INVALID_PARAMETERS) &&
       result == SDEI_SUCCESS)
    ==> handler_running(new_s) == false)
  && ((!(result == SDEI_INVALID_PARAMETERS) &&
       result != SDEI_SUCCESS)
    ==> handler_running(new_s) == handler_running(old_s))
  && (result != SDEI_SUCCESS
    ==> handler_running(new_s) == handler_running(old_s))
}