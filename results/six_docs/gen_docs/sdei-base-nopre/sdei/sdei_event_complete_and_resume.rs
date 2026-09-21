pub open spec fn sdei_event_complete_and_resume_spec(result: int64, old_s: S, new_s: S) -> bool {
    (result == SDEI_ERROR_NOT_SUPPORTED ==> !SdeiSupported(old_s))
    && (result == SDEI_ERROR_INVALID_PARAMETERS ==> !IsResumeAddressValid(old_s, new_s.resume_addr))
    && (result == SDEI_ERROR_DENIED ==> !SdeiHandlerRunning(old_s, new_s.pe_id))
    && (result == SDEI_SUCCESS ==> SdeiHandlerRunning(old_s, new_s.pe_id) && new_s.handler_running == false)
}