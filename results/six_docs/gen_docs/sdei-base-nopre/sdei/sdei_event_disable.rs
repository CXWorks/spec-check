pub open spec fn sdei_event_disable_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> !EventRegistered(old_s, event))
    && (result == RSI_ERROR_STATE ==> EventHandlerState(old_s, event) == HandlerUnregisterPending)
    && (result == RSI_SUCCESS ==> EventEnabled(old_s, event) && !EventEnabled(new_s, event))
}