pub open spec fn sdei_event_enable_spec(result: i32, old_s: S, new_s: S) -> bool {
    (result == SDEI_EVENT_ENABLE_NOT_SUPPORTED ==> !SdeiSupported(old_s))
    && (result == SDEI_EVENT_ENABLE_INVALID_PARAMETERS ==> !EventRegistered(old_s, event))
    && (result == SDEI_EVENT_ENABLE_DENIED ==> EventHandlerState(old_s, event) == HANDLER_UNREGISTER_PENDING)
    && (result == SDEI_EVENT_ENABLE_SUCCESS ==> EventEnabled(new_s, event) && EventEnabled(old_s, event) == false)
}