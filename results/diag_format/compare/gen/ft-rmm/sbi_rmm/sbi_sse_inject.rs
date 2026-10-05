pub open spec fn sbi_sse_inject_spec(event_id: UInt32, hart_id: unsigned long, result: SbiReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_SUCCESS ==> EventInjectAllowedByAttribute(new_s, event_id))
  && (result == SBI_SUCCESS && IsLocalEvent(new_s, event_id) ==> EventInjected(new_s, event_id, hart_id))
  && (result == SBI_SUCCESS && IsGlobalEvent(new_s, event_id) ==> EventInjected(new_s, event_id))
  && (result == SBI_SUCCESS && InSseHandler(new_s) && EventReady(new_s, event_id) && EventPriority(new_s, event_id) > EventPriority(new_s, CurrentEvent()) ==> EventHandledImmediately(new_s, event_id))
  && (result == SBI_SUCCESS && InSseHandler(new_s) && EventReady(new_s, event_id) && EventPriority(new_s, event_id) < EventPriority(new_s, CurrentEvent()) ==> EventRunsAfterCompletion(new_s, event_id, CurrentEvent(new_s)))
  && ((!(result == SBI_SUCCESS))
    ==> EventInjectAllowedByAttribute(new_s, event_id))
  && ((!(result == SBI_SUCCESS))
    ==> EventInjected(new_s, event_id, hart_id))
  && ((!(result == SBI_SUCCESS))
    ==> EventInjected(new_s, event_id))
  && ((!(result == SBI_SUCCESS))
    ==> EventHandledImmediately(new_s, event_id))
  && ((!(result == SBI_SUCCESS))
    ==> EventRunsAfterCompletion(new_s, event_id, CurrentEvent(new_s)))
  && (result != SBI_SUCCESS
    ==> EventAt(new_s, event_id, hart_id).injected == EventAt(old_s, event_id, hart_id).injected)
}