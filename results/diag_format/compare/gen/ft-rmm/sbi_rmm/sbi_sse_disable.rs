pub open spec fn sbi_sse_disable_spec(event_id: uint32_t, ret: struct sbiret, old_s: S, new_s: S) -> bool {
  (SseEventState(old_s, event_id) != ENABLED ==> ResultIsError(ret))
  && (ResultIsSuccess(ret) ==> ResultIsSuccess(ret))
  && (ResultIsSuccess(ret) && IsLocalEvent(old_s, event_id) ==> SseEventState(new_s, event_id, CallingHart()) == REGISTERED)
  && (ResultIsSuccess(ret) && IsGlobalEvent(old_s, event_id) ==> (forall (hart: int), SseEventState(new_s, event_id, hart) == REGISTERED))
  && ((!(SseEventState(old_s, event_id) != ENABLED))
    ==> ResultIsSuccess(ret))
  && (ResultIsError(ret)
    ==> SseEventState(new_s, event_id, CallingHart()) == SseEventState(old_s, event_id, CallingHart()))
  && (ResultIsError(ret)
    ==> (forall (hart: int), SseEventState(new_s, event_id, hart) == SseEventState(old_s, event_id, hart)))
}