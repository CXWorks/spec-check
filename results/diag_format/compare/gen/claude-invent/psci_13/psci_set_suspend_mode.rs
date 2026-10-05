pub open spec fn psci_set_suspend_mode_spec(result: PsciReturnCode, mode: UInt32, old_s: S, new_s: S) -> bool {
    (!PsciSetSuspendModeSupported(old_s) ==> result == PSCI_NOT_SUPPORTED)
    && ((PsciSetSuspendModeSupported(old_s) && mode != 0u32 && mode != 1u32) ==> result == PSCI_INVALID_PARAMETERS)
    && ((PsciSetSuspendModeSupported(old_s)
            && mode == 1u32
            && PsciSuspendMode(old_s) == 0u32
            && (!AllCoresRunningOffOrDefaultSuspended(old_s) || AnyCoreCalledCpuSuspendSinceLastModeChangeOrBoot(old_s)))
        ==> result == PSCI_DENIED)
    && ((PsciSetSuspendModeSupported(old_s)
            && mode == 0u32
            && PsciSuspendMode(old_s) == 1u32
            && !AllCoresOtherThanCallerOff(old_s))
        ==> result == PSCI_DENIED)
    && ((result != PSCI_SUCCESS) ==> PsciSuspendMode(new_s) == PsciSuspendMode(old_s))
    && ((PsciSetSuspendModeSupported(old_s)
            && (mode == 0u32 || mode == 1u32)
            && (!(mode == 1u32 && PsciSuspendMode(old_s) == 0u32)
                || (AllCoresRunningOffOrDefaultSuspended(old_s) && !AnyCoreCalledCpuSuspendSinceLastModeChangeOrBoot(old_s)))
            && (!(mode == 0u32 && PsciSuspendMode(old_s) == 1u32)
                || AllCoresOtherThanCallerOff(old_s)))
        ==> (result == PSCI_SUCCESS && PsciSuspendMode(new_s) == mode))
}
