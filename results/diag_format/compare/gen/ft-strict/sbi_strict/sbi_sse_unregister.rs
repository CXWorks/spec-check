pub open spec fn sbi_sse_unregister_spec(event_id: UInt32, error: struct sbiret.error, old_s: S, new_s: S) -> bool {
  (EventState(old_s, event_id, CallingHart(old_s)) != REGISTERED ==> ResultIsError(error))
  && (ResultIsSuccess(error) ==> ResultIsSuccess(error))
  && (ResultIsSuccess(error) && IsLocalEvent(old_s, event_id) ==> EventState(new_s, event_id, CallingHart(new_s)) == UNUSED)
  && (ResultIsSuccess(error) && IsGlobalEvent(old_s, event_id) ==> (forall|h: Hart| EventState(new_s, event_id, h) == UNUSED))
  && (ResultIsSuccess(error) ==> !HasRegisteredHandler(new_s, event_id, CallingHart(new_s)))
  && ((!(EventState(old_s, event_id, CallingHart(old_s)) != REGISTERED))
    ==> ResultIsSuccess(error))
}