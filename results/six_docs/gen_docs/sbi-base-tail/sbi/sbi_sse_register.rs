pub open spec fn sbi_sse_register_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_STATE ==> !EventIsUnused(old_s, event_id))
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> (handler_entry_pc % 2 != 0 || !IsValidEventId(event_id)))
    && (result == SBI_SBI_ERR_NOT_SUPPORTED ==> !PlatformSupportsEvent(event_id))
    && (result == SBI_SBI_SUCCESS ==> EventIsUnused(old_s, event_id) && EventIsRegistered(new_s, event_id))
}