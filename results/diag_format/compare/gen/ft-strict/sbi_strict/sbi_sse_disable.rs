pub open spec fn sbi_sse_disable_spec(event_id: UInt32, error: sbiret, old_s: S, new_s: S) -> bool {
  (EventState(old_s, event_id, CallingHart(old_s)) != ENABLED ==> ResultIsError(error))
  && (ResultIsSuccess(error) ==> ResultIsSuccess(error))
  && (ResultIsSuccess(error) && IsLocalEvent(old_s, event_id) ==> EventState(new_s, event_id, CallingHart(new_s)) == REGISTERED)
  && (ResultIsSuccess(error) && IsGlobalEvent(old_s, event_id) ==> (forall|h: Hart| EventState(new_s, event_id, h) == REGISTERED))
  && ((!(EventState(old_s, event_id, CallingHart(old_s)) != ENABLED))
    ==> ResultIsSuccess(error))
}