pub open spec fn sbi_sse_unregister_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    let event_id = old_s.cmd_input.event_id;
    let is_local = IsLocalEvent(event_id);
    let is_global = IsGlobalEvent(event_id);
    let event_state_old = SseEventState(event_id);
    let event_state_new = SseEventState(event_id);
    let handler_registered_old = IsHandlerRegistered(event_id);
    let handler_registered_new = IsHandlerRegistered(event_id);

    // Failure condition: event must be REGISTERED
    (event_state_old != REGISTERED ==> result == RSI_ERROR_STATE)

    // Success condition: event must be local or global
    (is_local ==> (event_state_new == UNUSED && !handler_registered_new))
    && (is_global ==> (event_state_new == UNUSED && !handler_registered_new))
}