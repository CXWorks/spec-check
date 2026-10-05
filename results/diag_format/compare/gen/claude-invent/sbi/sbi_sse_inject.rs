pub open spec fn sbi_sse_inject_spec(event_id: UInt32, hart_id: UInt64, result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (!SseEventInjectionAllowed(old_s, event_id) ==> result != SBI_SUCCESS)
    && (result == SBI_SUCCESS ==> SseEventInjectionAllowed(old_s, event_id) && SseEventInjected(old_s, new_s, event_id, hart_id))
}
