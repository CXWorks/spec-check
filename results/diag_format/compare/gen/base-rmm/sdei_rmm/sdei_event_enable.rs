pub open spec fn sdei_event_enable_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsKnownEvent(event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsEventRegisteredByClient(event) ==> ResultEqual(result, DENIED))
    && (EventHandlerState(event) == HANDLER_UNREGISTER_PENDING ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (IsPrivateEvent(event) ==> IsEventEnabledForPe(event, CallingPe()) && IsSharedEvent(event) ==> IsEventEnabledForClient(event, CallingClient())))
}