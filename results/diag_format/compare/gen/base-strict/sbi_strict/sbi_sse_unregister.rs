pub open spec fn sbi_sse_unregister_spec(error: sbiret, old_s: S, new_s: S) -> bool {
    (EventState(old_s, event_id, CallingHart()) != REGISTERED ==> ResultIsError(error))
    && (ResultIsSuccess(error) ==> ResultIsSuccess(error))
    && (ResultIsSuccess(error) ==> (IsLocalEvent(event_id) ==> EventState(new_s, event_id, CallingHart()) == UNUSED))
    && (ResultIsSuccess(error) ==> (IsGlobalEvent(event_id) ==> (forall|h: Hart| EventState(new_s, event_id, h) == UNUSED)))
    && (ResultIsSuccess(error) ==> !HasRegisteredHandler(new_s, event_id, CallingHart()))
}