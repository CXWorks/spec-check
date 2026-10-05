pub open spec fn sdei_event_register_spec(result: i64, old_s: S, new_s: S) -> bool {
    (result == SDEI_ERROR_INVALID_PARAMETERS ==> (
        (old_s.sdei_event_state(old_s.event) != SDEI_EVENT_STATE_UNREGISTERED)
        || (old_s.sdei_event_state(old_s.event) == SDEI_EVENT_STATE_UNREGISTERED && old_s.sdei_event_handler(old_s.event) != ())
        || (old_s.sdei_event_state(old_s.event) == SDEI_EVENT_STATE_UNREGISTERED && old_s.sdei_event_handler(old_s.event) == () && old_s.sdei_event_handler(old_s.event) != new_s.sdei_event_handler(old_s.event))
    ))
    && (result == SDEI_ERROR_DENIED ==> (
        old_s.sdei_event_state(old_s.event) == SDEI_EVENT_STATE_HANDLER_REGISTERED
    ))
    && (result == SDEI_SUCCESS ==> (
        old_s.sdei_event_state(old_s.event) == SDEI_EVENT_STATE_UNREGISTERED
        && new_s.sdei_event_state(old_s.event) == SDEI_EVENT_STATE_HANDLER_REGISTERED
        && new_s.sdei_event_handler(old_s.event) == old_s.sdei_event_handler(old_s.event)
        && new_s.sdei_event_argument(old_s.event) == old_s.sdei_event_argument(old_s.event)
    ))
}