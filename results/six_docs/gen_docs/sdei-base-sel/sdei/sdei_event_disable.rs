pub open spec fn sdei_event_disable_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SDEI_INVALID_PARAMETERS ==> (old_s.event_registered(old_s.event) == false))
    && (result == SDEI_DENIED ==> (old_s.event_handler_state(old_s.event) == SDEI_HANDLER_UNREGISTER_PENDING))
    && (result == SDEI_NOT_SUPPORTED ==> (old_s.sdei_supported == false))
    && (result == SDEI_SUCCESS ==> (old_s.event_registered(old_s.event) == true) && (new_s.event_registered(old_s.event) == true) && (new_s.event_handler_state(old_s.event) == SDEI_HANDLER_DISABLED))
}