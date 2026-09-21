pub open spec fn sbi_sse_disable_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_STATE ==> !EventEnabled(old_s, old_s.event_id))
    && (result == SBI_SBI_SUCCESS ==> EventEnabled(old_s, old_s.event_id) && EventRegistered(new_s, old_s.event_id))
}