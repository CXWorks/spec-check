pub open spec fn sbi_sse_disable_spec(event_id: UInt32, result: SbiRet, old_s: S, new_s: S) -> bool {
    (SseEventState(old_s, event_id, CallingHart(old_s)) != SSE_EVENT_ENABLED ==> (result.error != SBI_SUCCESS && new_s == old_s))
    && (SseEventState(old_s, event_id, CallingHart(old_s)) == SSE_EVENT_ENABLED ==> (
        result.error == SBI_SUCCESS
        && (IsSseLocalEvent(event_id) ==> (
            SseEventState(new_s, event_id, CallingHart(old_s)) == SSE_EVENT_REGISTERED
            && (forall|h: UInt64| h != CallingHart(old_s) ==> SseEventState(new_s, event_id, h) == SseEventState(old_s, event_id, h))
        ))
        && (!IsSseLocalEvent(event_id) ==> (
            forall|h: UInt64| SseEventState(new_s, event_id, h) == SSE_EVENT_REGISTERED
        ))
    ))
}
