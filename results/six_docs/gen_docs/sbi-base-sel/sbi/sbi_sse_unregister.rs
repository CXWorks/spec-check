pub open spec fn sbi_sse_unregister_spec(result: int, old_s: S, new_s: S) -> bool {
    (old_s.event_state(old_s.event_id) == REGISTERED ==> result == SBI_SBI_ERR_INVALID_STATE)
    && (result == SBI_SBI_SUCCESS ==> new_s.event_state(old_s.event_id) == UNUSED)
    && (result != SBI_SBI_SUCCESS && result != SBI_SBI_ERR_INVALID_STATE ==> new_s.event_state(old_s.event_id) == old_s.event_state(old_s.event_id))
}