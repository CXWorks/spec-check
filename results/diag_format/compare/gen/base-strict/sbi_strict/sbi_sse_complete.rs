pub open spec fn sbi_sse_complete_spec(error: SbiReturnCode, old_s: S, new_s: S) -> bool {
    (true ==> ResultEqual(error, SBI_SUCCESS))
    && (!PreExistsRunningEvent(old_s, CallingHart()) ==> HartSseStateUnchanged(old_s, CallingHart()))
    && (PreExistsRunningEvent(old_s, CallingHart()) && IsOneShot(PreHighestPriorityRunningEvent(old_s, CallingHart())) ==> PreHighestPriorityRunningEvent(old_s, CallingHart()).state == REGISTERED)
    && (PreExistsRunningEvent(old_s, CallingHart()) && !IsOneShot(PreHighestPriorityRunningEvent(old_s, CallingHart())) ==> PreHighestPriorityRunningEvent(old_s, CallingHart()).state == ENABLED)
    && (PreExistsRunningEvent(old_s, CallingHart()) ==> InterruptedSupervisorStateResumed(old_s, CallingHart(), PreHighestPriorityRunningEvent(old_s, CallingHart())))
}