pub open spec fn sdei_event_disable_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsKnownEvent(event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!EventIsRegisteredByClient(event) ==> ResultEqual(result, DENIED))
    && (EventHandlerState(event) == HANDLER_UNREGISTER_PENDING ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (EventIsPrivate(event) ==> !EventIsEnabledForPe(event, CallingPe()) && EventIsShared(event) ==> !EventIsEnabledForClient(event, CallingClient())))
}