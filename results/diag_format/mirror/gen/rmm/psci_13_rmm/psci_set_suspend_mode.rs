pub open spec fn psci_set_suspend_mode_spec(mode: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!PsciSetSuspendModeImplemented(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && ((mode != 0) && (mode != 1) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsOsInitiatedMode(mode) && CurrentCoordinationMode(old_s) == PLATFORM_COORDINATED && ExistsCore(old_s, c, !IsRunning(c) && !IsOffViaCpuOffOrNotBooted(c) && !IsSuspendedViaCpuDefaultSuspend(c)) ==> ResultEqual(result, DENIED))
  && (IsOsInitiatedMode(mode) && CurrentCoordinationMode(old_s) == PLATFORM_COORDINATED && AnyCoreCalledCpuSuspendSinceLastModeChangeOrBoot(old_s) ==> ResultEqual(result, DENIED))
  && (IsPlatformCoordinatedMode(mode) && CurrentCoordinationMode(old_s) == OS_INITIATED && ExistsCore(old_s, c, c != CallingCore() && !IsOffViaCpuOffOrNotBooted(c)) ==> ResultEqual(result, DENIED))
  && (result == RSI_SUCCESS ==> CurrentCoordinationMode(new_s) == mode)
  && ((PsciSetSuspendModeImplemented(old_s) &&
       !((mode != 0) && (mode != 1)) &&
       !(IsOsInitiatedMode(mode) && CurrentCoordinationMode(old_s) == PLATFORM_COORDINATED && ExistsCore(old_s, c, !IsRunning(c) && !IsOffViaCpuOffOrNotBooted(c) && !IsSuspendedViaCpuDefaultSuspend(c))) &&
       !(IsOsInitiatedMode(mode) && CurrentCoordinationMode(old_s) == PLATFORM_COORDINATED && AnyCoreCalledCpuSuspendSinceLastModeChangeOrBoot(old_s)) &&
       !(IsPlatformCoordinatedMode(mode) && CurrentCoordinationMode(old_s) == OS_INITIATED && ExistsCore(old_s, c, c != CallingCore() && !IsOffViaCpuOffOrNotBooted(c))))
    ==> CurrentCoordinationMode(new_s) == CurrentCoordinationMode(old_s))
}