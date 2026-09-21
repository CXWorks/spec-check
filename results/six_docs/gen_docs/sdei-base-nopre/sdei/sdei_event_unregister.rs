pub open spec fn sdei_event_unregister_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SDEI_SUCCESS ==> (old_s.sdei_event_registered(old_s.event) && !new_s.sdei_event_registered(new_s.event)))
    && (result == SDEI_NOT_SUPPORTED ==> !old_s.sdei_supported())
    && (result == SDEI_INVALID_PARAMETERS ==> !old_s.sdei_event_registered(old_s.event))
    && (result == SDEI_DENIED ==> old_s.sdei_event_registered(old_s.event) && !old_s.sdei_event_owned_by_client(old_s.event))
    && (result == SDEI_PENDING ==> old_s.sdei_event_registered(old_s.event) && old_s.sdei_event_handler_running(old_s.event))
}