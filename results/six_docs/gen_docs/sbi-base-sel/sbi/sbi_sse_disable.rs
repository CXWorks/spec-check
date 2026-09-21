pub open spec fn sbi_sse_disable_spec(result: int, old_s: S, new_s: S) -> bool {
    (old_s.sse_event_state(old_s.sse_event_id) == ENABLED ==> result == SBI_SBI_ERR_INVALID_STATE)
    && (result == SBI_SBI_SUCCESS ==> new_s.sse_event_state(old_s.sse_event_id) == REGISTERED)
    && (result != SBI_SBI_SUCCESS ==> new_s.sse_event_state(old_s.sse_event_id) == old_s.sse_event_state(old_s.sse_event_id))
}