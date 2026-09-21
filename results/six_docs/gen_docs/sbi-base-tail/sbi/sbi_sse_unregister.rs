pub open spec fn sbi_sse_unregister_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_STATE ==> !EventRegistered(old_s, event_id))
    && (result == SBI_SBI_SUCCESS ==> EventRegistered(old_s, event_id) && !EventRegistered(new_s, event_id))
}