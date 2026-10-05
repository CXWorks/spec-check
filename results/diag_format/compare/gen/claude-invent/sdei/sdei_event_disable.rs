pub open spec fn sdei_event_disable_spec(result: i64, event: i32, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && ((SdeiIsSupported(old_s) && !SdeiEventIsValid(old_s, event)) ==> (result == INVALID_PARAMETERS && new_s == old_s))
    && ((SdeiIsSupported(old_s) && SdeiEventIsValid(old_s, event)
            && (!SdeiEventIsRegisteredByClient(old_s, event) || SdeiEventIsHandlerUnregisterPending(old_s, event)))
        ==> (result == DENIED && new_s == old_s))
    && ((SdeiIsSupported(old_s) && SdeiEventIsValid(old_s, event)
            && SdeiEventIsRegisteredByClient(old_s, event)
            && !SdeiEventIsHandlerUnregisterPending(old_s, event))
        ==> (result == SUCCESS
            && !SdeiEventIsEnabled(new_s, event)
            && SdeiEventIsRegisteredByClient(new_s, event)
            && SdeiEventIsPending(new_s, event) == SdeiEventIsPending(old_s, event)
            && SdeiEventIsRunning(new_s, event) == SdeiEventIsRunning(old_s, event)))
}
