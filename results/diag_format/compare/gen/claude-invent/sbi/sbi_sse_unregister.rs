pub open spec fn sbi_sse_unregister_spec(result: SbiRet, event_id: UInt32, hart_id: UInt64, old_s: S, new_s: S) -> bool {
    (SseEventState(old_s, event_id, hart_id) != SSE_STATE_REGISTERED ==> (
        result.error != SBI_SUCCESS
        && new_s == old_s
    ))
    && (SseEventState(old_s, event_id, hart_id) == SSE_STATE_REGISTERED ==> (
        result.error == SBI_SUCCESS
        && (!SseEventIsGlobal(old_s, event_id) ==> (
            SseEventState(new_s, event_id, hart_id) == SSE_STATE_UNUSED
            && (forall|h: UInt64| h != hart_id ==> SseEventState(new_s, event_id, h) == SseEventState(old_s, event_id, h))
        ))
        && (SseEventIsGlobal(old_s, event_id) ==> (
            forall|h: UInt64| SseEventState(new_s, event_id, h) == SSE_STATE_UNUSED
        ))
    ))
}
