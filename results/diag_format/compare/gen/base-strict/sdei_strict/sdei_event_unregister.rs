pub open spec fn sdei_event_unregister_spec(result: Int64, event: Int32, old_s: S, new_s: S) -> bool {
    (!IsSdeiSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidEventNumber(event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsEventRegisteredByClient(event) ==> ResultEqual(result, DENIED))
    && (IsHandlerRunning(event) || IsUnregisterPending(event) ==> ResultEqual(result, PENDING))
    && (ResultEqual(result, SUCCESS) ==> (IsSharedEvent(event) ==> !IsEventRegisteredGlobally(event)))
    && (ResultEqual(result, SUCCESS) ==> (IsPrivateEvent(event) ==> !IsEventRegisteredOnPe(event, CurrentPe())))
    && (ResultEqual(result, SUCCESS) ==> !IsEventDeliverableToClient(event))
    && (EventHandlerState(event, old_s) == EventHandlerState(event, new_s))
}