pub open spec fn sdei_event_status_spec(result: Int64, event: Int32, old_s: S, new_s: S) -> bool {
    (!IsSdeiSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidEventNumber(event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (IsSdeiSupported() && IsValidEventNumber(event) ==> (
        (result as int) & 0xFFFFFFFFFFFFFFFC == 0
        && ((result as int) >> 2) == (EventHandlerIsRunning(event) as int)
        && ((result as int) >> 1) == (EventHandlerIsEnabled(event) as int)
        && ((result as int) >> 0) == (EventHandlerIsRegistered(event) as int)
    ))
    && (old_s == new_s)
}