pub open spec fn sbi_sse_inject_spec(result: SbiReturnCode, old_s: S, new_s: S) -> bool {
    (ResultEqual(result, SBI_SUCCESS) ==> EventInjectAllowedByAttribute(old_s.event_id))
    && (ResultEqual(result, SBI_SUCCESS) ==> (IsLocalEvent(old_s.event_id) ==> EventInjected(old_s.event_id, old_s.hart_id)))
    && (ResultEqual(result, SBI_SUCCESS) ==> (IsGlobalEvent(old_s.event_id) ==> EventInjected(old_s.event_id)))
    && (InSseHandler(old_s) && EventReady(old_s.event_id) && EventPriority(old_s.event_id) > EventPriority(CurrentEvent(old_s)) ==> EventHandledImmediately(old_s.event_id))
    && (InSseHandler(old_s) && EventReady(old_s.event_id) && EventPriority(old_s.event_id) < EventPriority(CurrentEvent(old_s)) ==> EventRunsAfterCompletion(old_s.event_id, CurrentEvent(old_s)))
    && (EventAt(old_s, old_s.event_id, old_s.hart_id).injected == EventAt(new_s, new_s.event_id, new_s.hart_id).injected)
    && (EventAt(old_s, old_s.event_id).injected == EventAt(new_s, new_s.event_id).injected)
}