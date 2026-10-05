pub open spec fn sbi_sse_complete_spec(error: SbiReturnCode, old_s: S, new_s: S) -> bool {
  (error == SBI_SUCCESS)
  && (!PreExistsRunningEvent(old_s, CallingHart(old_s)) ==> HartSseStateUnchanged(old_s, CallingHart(old_s)))
  && (PreExistsRunningEvent(old_s, CallingHart(old_s)) && IsOneShot(PreHighestPriorityRunningEvent(old_s, CallingHart(old_s))) ==> PreHighestPriorityRunningEvent(old_s, CallingHart(old_s)).state == REGISTERED)
  && (PreExistsRunningEvent(old_s, CallingHart(old_s)) && !IsOneShot(PreHighestPriorityRunningEvent(old_s, CallingHart(old_s))) ==> PreHighestPriorityRunningEvent(old_s, CallingHart(old_s)).state == ENABLED)
  && (PreExistsRunningEvent(old_s, CallingHart(old_s)) ==> InterruptedSupervisorStateResumed(old_s, CallingHart(old_s), PreHighestPriorityRunningEvent(old_s, CallingHart(old_s))))
  && ((!(error == SBI_SUCCESS))
    ==> HartSseStateUnchanged(new_s, CallingHart(new_s)))
  && ((!(error == SBI_SUCCESS))
    ==> PreHighestPriorityRunningEvent(new_s, CallingHart(new_s)).state == PreHighestPriorityRunningEvent(old_s, CallingHart(old_s)).state)
  && ((!(error == SBI_SUCCESS))
    ==> PreHighestPriorityRunningEvent(new_s, CallingHart(new_s)).state == PreHighestPriorityRunningEvent(old_s, CallingHart(old_s)).state)
  && (error != SBI_SUCCESS
    ==> SupervisorState(new_s, CallingHart(new_s)) == SupervisorState(old_s, CallingHart(old_s)))
}