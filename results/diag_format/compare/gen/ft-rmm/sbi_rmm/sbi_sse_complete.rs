pub open spec fn sbi_sse_complete_spec(result: struct sbiret, old_s: S, new_s: S) -> bool {
  (result == SBI_SUCCESS)
  && (!HasEventInState(old_s, current_hart, RUNNING) ==> no event state is modified)
  && (HasEventInState(old_s, current_hart, RUNNING) && IsOneShot(old_s, HighestPriorityRunningEvent(old_s, current_hart)) ==> HighestPriorityRunningEvent(new_s, current_hart).state == REGISTERED)
  && (HasEventInState(old_s, current_hart, RUNNING) && !IsOneShot(old_s, HighestPriorityRunningEvent(old_s, current_hart)) ==> HighestPriorityRunningEvent(new_s, current_hart).state == ENABLED)
  && (HasEventInState(old_s, current_hart, RUNNING) ==> the interrupted supervisor state is resumed, as described in Section 17.6)
  && ((!(result == SBI_SUCCESS))
    ==> HighestPriorityRunningEvent(new_s, current_hart).state == HighestPriorityRunningEvent(old_s, current_hart).state)
}