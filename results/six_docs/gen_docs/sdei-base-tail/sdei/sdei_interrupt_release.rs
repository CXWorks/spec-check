pub open spec fn sdei_interrupt_release_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SDEI_INVALID_PARAMETERS ==> (old_s.event_bound(old_s.event) == false))
    && (result == SDEI_DENIED ==> (old_s.event_handler_state(old_s.event) != SDEI_HANDLER_UNREGISTERED))
    && (result == SDEI_SUCCESS ==> (old_s.event_bound(old_s.event) == true) && (new_s.event_bound(old_s.event) == false))
}