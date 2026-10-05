pub open spec fn sdei_event_enable_spec(event: i32, result: i64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && ((SdeiIsSupported(old_s) && !SdeiEventIsKnown(old_s, event))
        ==> (result == INVALID_PARAMETERS && new_s == old_s))
    && ((SdeiIsSupported(old_s) && SdeiEventIsKnown(old_s, event)
        && (!SdeiEventIsRegisteredByCaller(old_s, event)
            || SdeiEventHandlerIsUnregisterPending(old_s, event)))
        ==> (result == DENIED && new_s == old_s))
    && ((SdeiIsSupported(old_s) && SdeiEventIsKnown(old_s, event)
        && SdeiEventIsRegisteredByCaller(old_s, event)
        && !SdeiEventHandlerIsUnregisterPending(old_s, event))
        ==> (result == SUCCESS
            && (SdeiEventIsEnabledForCaller(old_s, event) ==> new_s == old_s)
            && SdeiEventIsEnabledForCaller(new_s, event)
            && SdeiEventIsRegisteredByCaller(new_s, event)
            && (SdeiEventIsPrivate(old_s, event) ==> SdeiEventEnabledOnlyForCallingPe(old_s, new_s, event))
            && (!SdeiEventIsPrivate(old_s, event) ==> SdeiEventEnabledGloballyForCallingClient(old_s, new_s, event))
            && SdeiOtherEventsUnchanged(old_s, new_s, event)))
}
