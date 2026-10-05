pub open spec fn sdei_event_unregister_spec(event: i32, result: i64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && ((SdeiIsSupported(old_s) && !SdeiEventIsValid(old_s, event)) ==> (result == INVALID_PARAMETERS && new_s == old_s))
    && ((SdeiIsSupported(old_s)
        && SdeiEventIsValid(old_s, event)
        && !SdeiEventIsRegistered(old_s, event)
        && !SdeiEventIsUnregisterPending(old_s, event)) ==> (result == DENIED && new_s == old_s))
    && ((SdeiIsSupported(old_s)
        && SdeiEventIsValid(old_s, event)
        && (SdeiEventIsRegistered(old_s, event) || SdeiEventIsUnregisterPending(old_s, event))
        && (SdeiEventHandlerRunning(old_s, event) || SdeiEventIsUnregisterPending(old_s, event))) ==> (
            result == PENDING
            && SdeiEventIsUnregisterPending(new_s, event)
            && SdeiEventHandlerRunning(new_s, event) == SdeiEventHandlerRunning(old_s, event)
        ))
    && ((SdeiIsSupported(old_s)
        && SdeiEventIsValid(old_s, event)
        && SdeiEventIsRegistered(old_s, event)
        && !SdeiEventIsUnregisterPending(old_s, event)
        && !SdeiEventHandlerRunning(old_s, event)) ==> (
            result == SUCCESS
            && !SdeiEventIsRegistered(new_s, event)
            && !SdeiEventIsUnregisterPending(new_s, event)
            && !SdeiEventHandlerRunning(new_s, event)
        ))
}
