pub open spec fn sdei_event_routing_set_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SDEI_DENIED ==> (old_s.sdei_event_state(old_s.event) != SDEI_EVENT_HANDLER_REGISTERED))
    && (result == SDEI_INVALID_PARAMETERS ==> (old_s.event < 0 || old_s.event >= 0x100000000 || old_s.is_shared_event(old_s.event) == false || (old_s.routing_mode(old_s.event) != 0 && old_s.routing_mode(old_s.event) != 1) || (old_s.routing_mode(old_s.event) == 1 && old_s.affinity(old_s.event) < 0 || old_s.affinity(old_s.event) >= (1u64 << 48))))
    && (result == SDEI_NOT_SUPPORTED ==> true)
    && (result == SDEI_OUT_OF_RESOURCE ==> true)
    && (result == SDEI_PENDING ==> true)
    && (result == SDEI_SUCCESS ==> (old_s.event >= 0 && old_s.event < 0x100000000 && old_s.is_shared_event(old_s.event) && old_s.routing_mode(old_s.event) == 0 || old_s.routing_mode(old_s.event) == 1 && old_s.affinity(old_s.event) >= 0 && old_s.affinity(old_s.event) < (1u64 << 48) && old_s.sdei_event_state(old_s.event) == SDEI_EVENT_HANDLER_REGISTERED && new_s.event == old_s.event && new_s.routing_mode(old_s.event) == old_s.routing_mode(old_s.event) && new_s.affinity(old_s.event) == old_s.affinity(old_s.event) && new_s.sdei_event_state(old_s.event) == SDEI_EVENT_HANDLER_REGISTERED))
}