pub open spec fn sdei_event_unregister_spec(fid: UInt32, event: Int32, result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (SdeiIsSupported(old_s)
        && !IsValidEventNumber(old_s, event)
        ==> ResultEqual(result, INVALID_PARAMETERS))
    && (SdeiIsSupported(old_s)
        && IsValidEventNumber(old_s, event)
        && !EventIsRegisteredByClient(old_s, event)
        ==> ResultEqual(result, DENIED))
    && (SdeiIsSupported(old_s)
        && IsValidEventNumber(old_s, event)
        && EventIsRegisteredByClient(old_s, event)
        && (EventHandler(old_s, event).handler_running == TRUE
            || EventHandler(old_s, event).state == HANDLER_UNREGISTER_PENDING)
        ==> ResultEqual(result, PENDING))
    && (SdeiIsSupported(old_s)
        && IsValidEventNumber(old_s, event)
        && EventIsRegisteredByClient(old_s, event)
        && !(EventHandler(old_s, event).handler_running == TRUE
            || EventHandler(old_s, event).state == HANDLER_UNREGISTER_PENDING)
        ==> ResultEqual(result, SUCCESS)
            && (IsSharedEvent(old_s, event) ==> !EventIsRegisteredByClient(new_s, event))
            && (IsPrivateEvent(old_s, event) ==> !EventIsRegisteredByClient(new_s, event)))
}
