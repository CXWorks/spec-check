pub open spec fn sdei_event_enable_spec(event: Int32, result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsKnownEvent(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsEventRegisteredByClient(old_s, event) ==> ResultEqual(result, DENIED))
    && (EventHandlerState(old_s, event) == HANDLER_UNREGISTER_PENDING ==> ResultEqual(result, DENIED))
    && ((SdeiIsSupported(old_s)
        && IsKnownEvent(old_s, event)
        && IsEventRegisteredByClient(old_s, event)
        && EventHandlerState(old_s, event) != HANDLER_UNREGISTER_PENDING)
        ==> (ResultEqual(result, SUCCESS)
            && (IsPrivateEvent(old_s, event) ==> IsEventEnabledForPe(new_s, event, CallingPe(old_s)))
            && (IsSharedEvent(old_s, event) ==> IsEventEnabledForClient(new_s, event, CallingClient(old_s)))))
}
