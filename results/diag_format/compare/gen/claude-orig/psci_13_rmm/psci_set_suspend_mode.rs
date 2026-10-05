pub open spec fn psci_set_suspend_mode_spec(result: PsciReturnCode, old_s: S, new_s: S, fid: UInt64, mode: UInt64) -> bool {
    (!PsciSetSuspendModeImplemented(old_s) ==> result == NOT_SUPPORTED)
    && ((mode != 0 && mode != 1) ==> result == INVALID_PARAMETERS)
    && ((IsOsInitiatedMode(mode)
        && CurrentCoordinationMode(old_s) == PLATFORM_COORDINATED
        && (exists|c: UInt64| !IsRunning(old_s, c) && !IsOffViaCpuOffOrNotBooted(old_s, c) && !IsSuspendedViaCpuDefaultSuspend(old_s, c)))
        ==> result == DENIED)
    && ((IsOsInitiatedMode(mode)
        && CurrentCoordinationMode(old_s) == PLATFORM_COORDINATED
        && AnyCoreCalledCpuSuspendSinceLastModeChangeOrBoot(old_s))
        ==> result == DENIED)
    && ((IsPlatformCoordinatedMode(mode)
        && CurrentCoordinationMode(old_s) == OS_INITIATED
        && (exists|c: UInt64| c != CallingCore(old_s) && !IsOffViaCpuOffOrNotBooted(old_s, c)))
        ==> result == DENIED)
    && ((PsciSetSuspendModeImplemented(old_s)
        && (mode == 0 || mode == 1)
        && !(IsOsInitiatedMode(mode)
            && CurrentCoordinationMode(old_s) == PLATFORM_COORDINATED
            && (exists|c: UInt64| !IsRunning(old_s, c) && !IsOffViaCpuOffOrNotBooted(old_s, c) && !IsSuspendedViaCpuDefaultSuspend(old_s, c)))
        && !(IsOsInitiatedMode(mode)
            && CurrentCoordinationMode(old_s) == PLATFORM_COORDINATED
            && AnyCoreCalledCpuSuspendSinceLastModeChangeOrBoot(old_s))
        && !(IsPlatformCoordinatedMode(mode)
            && CurrentCoordinationMode(old_s) == OS_INITIATED
            && (exists|c: UInt64| c != CallingCore(old_s) && !IsOffViaCpuOffOrNotBooted(old_s, c))))
        ==> ((IsOsInitiatedMode(mode) ==> CurrentCoordinationMode(new_s) == OS_INITIATED)
            && (IsPlatformCoordinatedMode(mode) ==> CurrentCoordinationMode(new_s) == PLATFORM_COORDINATED)))
    && (result != SUCCESS ==> CurrentCoordinationMode(new_s) == CurrentCoordinationMode(old_s))
}
