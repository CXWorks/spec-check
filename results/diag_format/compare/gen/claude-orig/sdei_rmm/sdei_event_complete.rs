pub open spec fn sdei_event_complete_spec(fid: UInt32, status_code: UInt32, result: Result<(), SdeiStatusCode>, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && ((SdeiIsSupported(old_s) && !HandlerRunning(old_s, CallingPe(old_s))) ==> ResultEqual(result, DENIED))
    && ((SdeiIsSupported(old_s) && HandlerRunning(old_s, CallingPe(old_s))) ==> (
        result.is_Ok()
        && HandlerRunning(new_s, CallingPe(old_s)) == false
    ))
}
