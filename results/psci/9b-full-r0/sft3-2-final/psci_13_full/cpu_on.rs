pub open spec fn cpu_on_spec(target_cpu: UInt32, entry_point_address: UInt32, context_id: UInt32, result: int, old_s: S, new_s: S) -> bool {
  (result == PSCI_INVALID_PARAMETERS ==> CpuIsValid(old_s, target_cpu as Bits64) == false)
  && (result == PSCI_INVALID_ADDRESS ==> AddrIsNonSecure(old_s, entry_point_address as Address) == false)
  && (result == PSCI_ALREADY_ON ==> CpuIsOn(old_s, target_cpu as Bits64))
  && (result == PSCI_ON_PENDING ==> CpuIsOnPending(old_s, target_cpu as Bits64))
  && ((!(result == PSCI_INVALID_PARAMETERS) &&
       result == PSCI_SUCCESS)
    ==> CpuIsOn(new_s, target_cpu as Bits64))
  && (result != PSCI_INVALID_PARAMETERS &&
       result != PSCI_INVALID_ADDRESS &&
       result != PSCI_ALREADY_ON &&
       result != PSCI_ON_PENDING &&
       result != PSCI_INTERNAL_FAILURE &&
       result != PSCI_DENIED
    ==> CpuIsOn(new_s, target_cpu as Bits64) == CpuIsOn(old_s, target_cpu as Bits64))
  && (result != PSCI_INVALID_PARAMETERS &&
       result != PSCI_INVALID_ADDRESS &&
       result != PSCI_ALREADY_ON &&
       result != PSCI_ON_PENDING &&
       result != PSCI_INTERNAL_FAILURE &&
       result != PSCI_DENIED
    ==> CpuIsOnPending(new_s, target_cpu as Bits64) == CpuIsOnPending(old_s, target_cpu as Bits64))
  && (result != PSCI_SUCCESS &&
       result != PSCI_ALREADY_ON &&
       result != PSCI_ON_PENDING
    ==> CpuIsOn(new_s, target_cpu as Bits64) == CpuIsOn(old_s, target_cpu as Bits64))
}