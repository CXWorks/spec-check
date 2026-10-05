pub open spec fn sdei_event_status_spec(result: Int64, event: Int32, old_s: S, new_s: S) -> bool {
    (!IsSdeiSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidEventNumber(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((IsSdeiSupported(old_s) && IsValidEventNumber(old_s, event)) ==> (
        ((result as u64) >> 3u64) == 0u64
        && (((result as u64) >> 2u64) & 1u64) == (if EventHandlerIsRunning(old_s, event) { 1u64 } else { 0u64 })
        && (((result as u64) >> 1u64) & 1u64) == (if EventHandlerIsEnabled(old_s, event) { 1u64 } else { 0u64 })
        && ((result as u64) & 1u64) == (if EventHandlerIsRegistered(old_s, event) { 1u64 } else { 0u64 })
        && new_s == old_s
    ))
}
