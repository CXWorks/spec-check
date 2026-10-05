pub open spec fn sbi_sse_disable_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (event_enabled(old_s, event_id) ==> result.code == SBI_SSE_SUCCESS)
    && (result.code != SBI_SSE_SUCCESS ==> !event_enabled(old_s, event_id))
    && (result.code == SBI_SSE_SUCCESS ==> event_registered(new_s, event_id))
    && (result.code == SBI_SSE_SUCCESS ==> !event_enabled(new_s, event_id))
}

fn event_enabled(s: S, event_id: uint32_t) -> bool {
    s.sse_events[event_id as usize].state == SBI_SSE_EVENT_ENABLED
}

fn event_registered(s: S, event_id: uint32_t) -> bool {
    s.sse_events[event_id as usize].state == SBI_SSE_EVENT_REGISTERED
}