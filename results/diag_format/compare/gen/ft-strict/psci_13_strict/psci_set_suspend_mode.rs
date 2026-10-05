pub open spec fn psci_set_suspend_mode_spec(mode: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!IsFunctionImplemented(old_s, PSCI_SET_SUSPEND_MODE) ==> result == NOT_SUPPORTED)
  && (mode != 0 && mode != 1 ==> result == INVALID_PARAMETERS)
  && (CurrentSuspendMode(old_s) == PLATFORM_COORDINATED && SuspendModeOf(old_s, mode) == OS_INITIATED && !(forall|c: Core| CoreIsRunning(c) || CoreIsOffViaCpuOffOrNotBooted(c) || CoreIsSuspendedViaCpuDefaultSuspend(c)) ==> result == DENIED)
  && (CurrentSuspendMode(old_s) == PLATFORM_COORDINATED && SuspendModeOf(old_s, mode) == OS_INITIATED && (exists|c: Core| CpuSuspendCalledSinceLastModeChangeOrBoot(c)) ==> result == DENIED)
  && (CurrentSuspendMode(old_s) == OS_INITIATED && SuspendModeOf(old_s, mode) == PLATFORM_COORDINATED && !(forall|c: Core| c != CallingCore() ==> CoreIsOffViaCpuOffOrNotBooted(c)) ==> result == DENIED)
  && (result == SUCCEEDED ==> CurrentSuspendMode(new_s) == SuspendModeOf(new_s, mode))
  && ((IsFunctionImplemented(old_s, PSCI_SET_SUSPEND_MODE) &&
       !(mode != 0 && mode != 1) &&
       !(CurrentSuspendMode(old_s) == PLATFORM_COORDINATED && SuspendModeOf(old_s, mode) == OS_INITIATED && !(forall|c: Core| CoreIsRunning(c) || CoreIsOffViaCpuOffOrNotBooted(c) || CoreIsSuspendedViaCpuDefaultSuspend(c))) &&
       !(CurrentSuspendMode(old_s) == PLATFORM_COORDINATED && SuspendModeOf(old_s, mode) == OS_INITIATED && (exists|c: Core| CpuSuspendCalledSinceLastModeChangeOrBoot(c))) &&
       !(CurrentSuspendMode(old_s) == OS_INITIATED && SuspendModeOf(old_s, mode) == PLATFORM_COORDINATED && !(forall|c: Core| c != CallingCore() ==> CoreIsOffViaCpuOffOrNotBooted(c))))
    ==> result == SUCCEEDED)
  && (result != SUCCEEDED
    ==> CurrentSuspendMode(new_s) == CurrentSuspendMode(old_s))
}