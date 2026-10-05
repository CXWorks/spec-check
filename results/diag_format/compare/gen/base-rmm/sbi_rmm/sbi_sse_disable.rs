pub open spec fn sbi_sse_disable_spec(ret: sbiret, old_s: S, new_s: S) -> bool {
    (SseEventState(old_s, event_id) != ENABLED ==> ResultIsError(ret))
    && (ResultIsSuccess(ret) ==> (ResultIsSuccess(ret) && (IsLocalEvent(event_id) ==> SseEventState(new_s, event_id, CallingHart()) == REGISTERED) && (IsGlobalEvent(event_id) ==> forall hart: SseEventState(new_s, event_id, hart) == REGISTERED)))
}