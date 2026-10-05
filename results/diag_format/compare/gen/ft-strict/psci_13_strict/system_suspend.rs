pub open spec fn system_suspend_spec(entry_point_address: Address, context_id: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!SystemSuspendImplemented(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (EntryPointKnownInvalid(old_s, entry_point_address) ==> ResultEqual(result, INVALID_ADDRESS))
  && (exists|c: Core| c != CallingCore(old_s) && AffinityState(c) != OFF ==> ResultEqual(result, DENIED))
  && (result == PSCI_SUCCESS ==> SystemInDeepestPowerdownState(new_s))
  && (result == PSCI_SUCCESS ==> CoreResumesAtEntryPoint(new_s, CallingCore(new_s), entry_point_address, context_id))
  && ((SystemSuspendImplemented(old_s) &&
       !EntryPointKnownInvalid(old_s, entry_point_address) &&
       !(exists|c: Core| c != CallingCore(old_s) && AffinityState(c) != OFF))
    ==> result == PSCI_SUCCESS)
  && (result != PSCI_SUCCESS
    ==> SystemInDeepestPowerdownState(new_s))
  && (result != PSCI_SUCCESS
    ==> CoreResumesAtEntryPoint(new_s, CallingCore(new_s), entry_point_address, context_id))
}