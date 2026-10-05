pub open spec fn sdei_event_status_spec(result: i64, event: i32, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> result == NOT_SUPPORTED)
    && ((SdeiIsSupported(old_s) && !SdeiEventIsValid(old_s, event)) ==> result == INVALID_PARAMETERS)
    && ((SdeiIsSupported(old_s) && SdeiEventIsValid(old_s, event)) ==> (
        ((result as u64) >> 3u64) == 0u64
        && (((((result as u64) >> 2u64) & 1u64) == 1u64) <==> SdeiEventHandlerRunning(old_s, event))
        && (((((result as u64) >> 1u64) & 1u64) == 1u64) <==> SdeiEventHandlerEnabled(old_s, event))
        && ((((result as u64) & 1u64) == 1u64) <==> SdeiEventHandlerRegistered(old_s, event))
    ))
    && new_s == old_s
}
