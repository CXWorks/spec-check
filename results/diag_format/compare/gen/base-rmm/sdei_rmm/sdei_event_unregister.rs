pub open spec fn sdei_event_unregister_spec(result: Int64, event: Int32, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidEventNumber(event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!EventIsRegisteredByClient(event) ==> ResultEqual(result, DENIED))
    && ((EventHandler(event).handler_running == TRUE || EventHandler(event).state == HANDLER_UNREGISTER_PENDING) ==> ResultEqual(result, PENDING))
    && (ResultEqual(result, SUCCESS) ==> (IsSharedEvent(event) ==> !EventIsRegisteredByClient(event)))
    && (ResultEqual(result, SUCCESS) ==> (IsPrivateEvent(event) ==> !EventIsRegisteredByClient(event)))
    && (ResultEqual(result, SUCCESS) ==> NoFurtherEventsDelivered(event))
}