pub open spec fn sbi_sse_enable_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_STATE ==> !EventRegistered(old_s, event_id))
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> !EventValid(old_s, event_id))
    && (result == SBI_SBI_ERR_NOT_SUPPORTED ==> !PlatformSupportsEvent(old_s, event_id))
    && (result == SBI_SBI_SUCCESS ==> EventRegistered(old_s, event_id) && EventEnabled(new_s, event_id))
}