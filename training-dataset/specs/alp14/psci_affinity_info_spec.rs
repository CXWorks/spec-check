pub open spec fn psci_affinity_info_spec(target_affinity: Bits64, lowest_affinity_level: UInt32, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (lowest_affinity_level != 0 ==> result == PSCI_INVALID_PARAMETERS)
  && (!MpidrIsUsed(old_s, target_affinity) ==> result == PSCI_INVALID_PARAMETERS)
  && (result == PSCI_SUCCESS && RecFromMpidr(old_s, target_affinity).flags.runnable == RUNNABLE ==> result == PSCI_SUCCESS)
  && (result == PSCI_SUCCESS && RecFromMpidr(old_s, target_affinity).flags.runnable == NOT_RUNNABLE ==> result == PSCI_OFF)
  && ((!(lowest_affinity_level != 0) &&
       MpidrIsUsed(old_s, target_affinity))
    ==> result == PSCI_SUCCESS)
}
