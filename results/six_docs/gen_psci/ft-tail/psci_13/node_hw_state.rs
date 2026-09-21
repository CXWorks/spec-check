pub open spec fn node_hw_state_spec(target_cpu: Bits64, power_level: UInt32, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (result == PSCI_NOT_SUPPORTED ==> true)
  && (result == PSCI_INVALID_PARAMETERS ==> true)
  && ((!(CpuIsValid(old_s, target_cpu)) ||
       !(0 <= power_level && power_level < 64))
    ==> result == PSCI_INVALID_PARAMETERS)
  && (result == PSCI_SUCCESS
    ==> true)
  && (result != PSCI_SUCCESS
    ==> CpuState(new_s, target_cpu) == CpuState(old_s, target_cpu))
}