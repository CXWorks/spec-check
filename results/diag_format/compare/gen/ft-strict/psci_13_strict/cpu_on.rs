pub open spec fn cpu_on_spec(target_cpu: Mpidr, entry_point_address: Address, context_id: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!IsValidMpidr(old_s, target_cpu) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsKnownUnavailableToCaller(old_s, entry_point_address) ==> ResultEqual(result, INVALID_ADDRESS))
  && (CoreState(old_s, target_cpu) == ON ==> ResultEqual(result, ALREADY_ON))
  && (CoreState(old_s, target_cpu) == ON_PENDING ==> ResultEqual(result, ON_PENDING))
  && (!CanBePhysicallyPoweredUp(old_s, target_cpu) ==> ResultEqual(result, INTERNAL_FAILURE))
  && (!IsAvailableForOsUsage(old_s, target_cpu) ==> ResultEqual(result, DENIED))
  && (result == PSCI_SUCCESS ==> CoreRestartsAtEntryPoint(new_s, target_cpu, entry_point_address))
  && (result == PSCI_SUCCESS ==> ContextIdPresentedAtReturnExceptionLevel(new_s, target_cpu, context_id))
  && (result == PSCI_SUCCESS ==> CachesInvalidatedOnBoot(new_s, target_cpu) || HardwareInvalidatesCachesOnBoot(new_s, target_cpu))
  && (result == PSCI_SUCCESS ==> CoherencyManaged(new_s, target_cpu))
  && ((IsValidMpidr(old_s, target_cpu) &&
       !IsKnownUnavailableToCaller(old_s, entry_point_address) &&
       !(CoreState(old_s, target_cpu) == ON) &&
       !(CoreState(old_s, target_cpu) == ON_PENDING) &&
       CanBePhysicallyPoweredUp(old_s, target_cpu) &&
       IsAvailableForOsUsage(old_s, target_cpu))
    ==> result == PSCI_SUCCESS)
  && (result != PSCI_SUCCESS
    ==> CoreState(new_s, target_cpu) == CoreState(old_s, target_cpu))
  && (result != PSCI_SUCCESS
    ==> CoreRestartsAtEntryPoint(new_s, target_cpu, entry_point_address) == false)
  && (result != PSCI_SUCCESS
    ==> ContextIdPresentedAtReturnExceptionLevel(new_s, target_cpu, context_id) == false)
  && (result != PSCI_SUCCESS
    ==> CachesInvalidatedOnBoot(new_s, target_cpu) == false)
  && (result != PSCI_SUCCESS
    ==> CoherencyManaged(new_s, target_cpu) == false)
  && (result != PSCI_SUCCESS
    ==> CorePowerState(new_s, target_cpu) == CorePowerState(old_s, target_cpu))
  && (result != PSCI_SUCCESS
    ==> CoreCaches(new_s, target_cpu) == CoreCaches(old_s, target_cpu))
}