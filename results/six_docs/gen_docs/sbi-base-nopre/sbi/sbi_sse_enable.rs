pub open spec fn sbi_sse_enable_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result == SBI_ERR_INVALID_STATE ==> !EventIsRegistered(old_s, event_id))
    && (result == SBI_ERR_INVALID_PARAM ==> !IsValidEventId(event_id))
    && (result == SBI_ERR_NOT_SUPPORTED ==> !PlatformSupportsEvent(event_id))
    && (result == SBI_SUCCESS ==> EventIsRegistered(old_s, event_id) && EventIsEnabled(new_s, event_id) && !EventIsEnabled(old_s, event_id))
}