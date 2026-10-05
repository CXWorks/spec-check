pub open spec fn sbi_sse_complete_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (ResultEqual(result, SBI_SUCCESS))
    && (!HasEventInState(old_s, current_hart) ==> (HasEventInState(new_s, current_hart) == false))
    && (HasEventInState(old_s, current_hart) && IsOneShot(HighestPriorityRunningEvent(old_s, current_hart)) ==> (HighestPriorityRunningEvent(new_s, current_hart).state == REGISTERED))
    && (HasEventInState(old_s, current_hart) && !IsOneShot(HighestPriorityRunningEvent(old_s, current_hart)) ==> (HighestPriorityRunningEvent(new_s, current_hart).state == ENABLED))
    && (HasEventInState(old_s, current_hart) ==> (ResumeSupervisorState(old_s, current_hart) == ResumeSupervisorState(new_s, current_hart)))
}