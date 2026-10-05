pub open spec fn sbi_sse_disable_spec(error: sbiret, old_s: S, new_s: S) -> bool {
    (EventState(event_id(old_s), CallingHart()) != ENABLED ==> ResultIsError(error))
    && (ResultIsSuccess(error) ==> ResultIsSuccess(error))
    && (ResultIsSuccess(error) ==> (IsLocalEvent(event_id(old_s)) ==> EventState(event_id(new_s), CallingHart()) == REGISTERED))
    && (ResultIsSuccess(error) ==> (IsGlobalEvent(event_id(old_s)) ==> (forall|h: Hart| EventState(event_id(new_s), h) == REGISTERED)))
}