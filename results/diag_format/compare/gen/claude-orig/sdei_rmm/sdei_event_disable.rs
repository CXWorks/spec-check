pub open spec fn sdei_event_disable_spec(fid: UInt32, event: Int32, result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsKnownEvent(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!EventIsRegisteredByClient(old_s, event) ==> ResultEqual(result, DENIED))
    && (EventHandlerState(old_s, event) == HANDLER_UNREGISTER_PENDING ==> ResultEqual(result, DENIED))
    && ((SdeiIsSupported(old_s)
        && IsKnownEvent(old_s, event)
        && EventIsRegisteredByClient(old_s, event)
        && EventHandlerState(old_s, event) != HANDLER_UNREGISTER_PENDING)
        ==> (ResultEqual(result, SUCCESS)
            && (EventIsPrivate(old_s, event) ==> !EventIsEnabledForPe(new_s, event, CallingPe(old_s)))
            && (EventIsShared(old_s, event) ==> !EventIsEnabledForClient(new_s, event, CallingClient(old_s)))))
}
