pub open spec fn node_hw_state_spec(target_cpu: UInt64, power_level: UInt64, result: Result<(), int>, old_s: S, new_s: S) -> bool {
  (result == PSCI_NOT_SUPPORTED ==> CpuIsValid(new_s, target_cpu))
  && (result == PSCI_INVALID_PARAMETERS ==> CpuIsValid(new_s, target_cpu))
  && ((!(result == PSCI_NOT_SUPPORTED) &&
       !(result == PSCI_INVALID_PARAMETERS))
    ==> result == PSCI_SUCCESS)
  && (result != PSCI_SUCCESS
    ==> CpuIsOn(new_s, target_cpu) == CpuIsOn(old_s, target_cpu))
}