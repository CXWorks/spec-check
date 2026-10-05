pub open spec fn system_suspend_spec(entry_point_address: Address, context_id: UInt64, result: PSCI return code, old_s: S, new_s: S) -> bool {
  (!SystemSuspendImplemented(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (IsKnownUnavailableToCaller(old_s, entry_point_address) ==> ResultEqual(result, INVALID_ADDRESS))
  && (Exists(core : core != CallingCore(old_s) && CoreState(old_s, core) != OFF) ==> ResultEqual(result, DENIED))
  && (result == PSCI_SUCCESS ==> SystemPowerState(new_s) == DeepestPlatformPowerdownState(new_s))
  && (result == PSCI_SUCCESS ==> OnWakeup(new_s, CallingCore(new_s) resumes execution at entry_point_address))
  && ((SystemSuspendImplemented(old_s) &&
       !IsKnownUnavailableToCaller(old_s, entry_point_address) &&
       !(Exists(core : core != CallingCore(old_s) && CoreState(old_s, core) != OFF)))
    ==> result == PSCI_SUCCESS)
  && (result != PSCI_SUCCESS
    ==> SystemPowerState(new_s) == SystemPowerState(old_s))
  && (!(result == PSCI_SUCCESS)
    ==> OnWakeup(new_s, CallingCore(new_s) resumes execution at entry_point_address))
}