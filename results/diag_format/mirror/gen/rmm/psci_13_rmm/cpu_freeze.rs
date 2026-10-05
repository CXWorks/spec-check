pub open spec fn cpu_freeze_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!IsImplemented(old_s, CPU_FREEZE) ==> ResultEqual(result, NOT_SUPPORTED))
  && (CpuOffWouldBeDenied(old_s, CurrentCore()) ==> ResultEqual(result, DENIED))
  && (result == PSCI_SUCCESS ==> CoreAt(new_s, CurrentCore()).power_state == IMPLEMENTATION_DEFINED_LOW_POWER_STATE)
  && (result == PSCI_SUCCESS ==> !WakeupInterruptsResumeExecution(new_s, CurrentCore()))
  && (result == PSCI_SUCCESS ==> InterruptsRemainPendingOrActive(new_s, CurrentCore()))
  && ((IsImplemented(old_s, CPU_FREEZE) &&
       !CpuOffWouldBeDenied(old_s, CurrentCore()))
    ==> result == PSCI_SUCCESS)
  && (result != PSCI_SUCCESS
    ==> CoreAt(new_s, CurrentCore()).power_state == CoreAt(old_s, CurrentCore()).power_state)
  && (result != PSCI_SUCCESS
    ==> WakeupInterruptsResumeExecution(new_s, CurrentCore()) == WakeupInterruptsResumeExecution(old_s, CurrentCore()))
  && (result != PSCI_SUCCESS
    ==> InterruptsRemainPendingOrActive(new_s, CurrentCore()) == InterruptsRemainPendingOrActive(old_s, CurrentCore()))
}