pub open spec fn migrate_spec(target_cpu: Bits64, result: int, old_s: S, new_s: S) -> bool {
  (result == PSCI_NOT_SUPPORTED ==> CpuIsOn(old_s, target_cpu))
  && (result == PSCI_INVALID_PARAMETERS ==> CpuIsOn(old_s, target_cpu))
  && (result == PSCI_DENIED ==> CpuIsOn(old_s, target_cpu))
  && (result == PSCI_NOT_PRESENT ==> CpuIsOn(old_s, target_cpu))
  && ((!(result == PSCI_NOT_SUPPORTED) &&
       !(result == PSCI_INVALID_PARAMETERS) &&
       !(result == PSCI_DENIED) &&
       !(result == PSCI_NOT_PRESENT))
    ==> CpuIsOn(new_s, target_cpu))
}