pub open spec fn sdei_event_complete_spec(result: int64, old_s: S, new_s: S) -> bool {
    (old_s.pe_handler_running == false ==> result == SDEI_ERROR_DENIED)
    && (result != SDEI_ERROR_NOT_SUPPORTED)
}