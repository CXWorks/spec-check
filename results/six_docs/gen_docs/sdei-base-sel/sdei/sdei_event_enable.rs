pub open spec fn sdei_event_enable_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SDEI_SUCCESS ==> (new_s.sdei_event_enabled(old_s.sdei_event_number) == true))
    && (result == SDEI_NOT_SUPPORTED ==> (old_s.sdei_supported == false))
    && (result == SDEI_INVALID_PARAMETERS ==> (old_s.sdei_event_number < 0 || old_s.sdei_event_number >= 0x10000))
    && (result == SDEI_DENIED ==> (old_s.sdei_event_registered(old_s.sdei_event_number) == false || old_s.sdei_event_handler_state(old_s.sdei_event_number) == SDEI_EVENT_HANDLER_UNREGISTER_PENDING))
    && (result != SDEI_SUCCESS && result != SDEI_NOT_SUPPORTED && result != SDEI_INVALID_PARAMETERS && result != SDEI_DENIED ==> false)
}