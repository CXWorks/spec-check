pub open spec fn sdei_event_routing_set_spec(result: int64, old_s: S, new_s: S) -> bool {
    (result == SDEI_ERROR_INVALID_PARAMETERS ==> (
        !old_s.sdei_event_registered(old_s.event)
        || !old_s.sdei_event_is_shared(old_s.event)
        || (old_s.routing_mode != 0 && old_s.routing_mode != 1)
        || (old_s.routing_mode == 1 && !old_s.affinity_is_valid(old_s.affinity))
    ))
    && (result == SDEI_ERROR_DENIED ==> old_s.sdei_event_handler_state(old_s.event) != SDEI_EVENT_HANDLER_STATE_REGISTERED)
    && (result == SDEI_ERROR_NOT_SUPPORTED ==> !old_s.sdei_supported())
    && (result == SDEI_SUCCESS ==> (
        old_s.sdei_event_registered(old_s.event)
        && old_s.sdei_event_is_shared(old_s.event)
        && (old_s.routing_mode == 0 || (old_s.routing_mode == 1 && old_s.affinity_is_valid(old_s.affinity)))
        && old_s.sdei_event_handler_state(old_s.event) == SDEI_EVENT_HANDLER_STATE_REGISTERED
        && old_s.sdei_supported()
        && new_s.sdei_event_routing_mode(old_s.event) == old_s.routing_mode
        && new_s.sdei_event_affinity(old_s.event) == old_s.affinity
    ))
}