pub open spec fn sdei_event_disable_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidEventNumber(event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsEventRegisteredByClient(event, CallingClient()) ==> ResultEqual(result, DENIED))
    && (EventHandlerState(event) == HANDLER_UNREGISTER_PENDING ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (IsPrivateEvent(event) ==> !IsEventEnabledForPe(event, CallingPe())))
    && (ResultEqual(result, SUCCESS) ==> (IsSharedEvent(event) ==> !IsEventEnabledForClient(event, CallingClient())))
    && (ResultEqual(result, SUCCESS) ==> RunningEventHandlerUnaffected(event))
    && (ResultEqual(result, SUCCESS) ==> TriggeredEventsStayPendingUntilEnabled(event))
}