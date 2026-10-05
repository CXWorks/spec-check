pub open spec fn sdei_event_complete_spec(status_code: UInt32, result: i64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && ((SdeiIsSupported(old_s) && !SdeiHandlerRunning(old_s)) ==> (result == DENIED && new_s == old_s))
    && ((SdeiIsSupported(old_s) && SdeiHandlerRunning(old_s)) ==> (
        !SdeiHandlerRunning(new_s)
        && SdeiEventHandlingCompleted(old_s, new_s, status_code)
        && SdeiResumesInterruptedContext(old_s, new_s)
    ))
}
