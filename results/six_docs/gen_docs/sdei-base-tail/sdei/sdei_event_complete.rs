pub open spec fn sdei_event_complete_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SDEI_DENIED ==> old_s.handler_running == false)
    && (result != SDEI_DENIED ==> old_s.handler_running == true)
    && (result != SDEI_DENIED ==> new_s.handler_running == false)
}