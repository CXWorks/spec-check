pub open spec fn migrate_spec(result: int, old_s: S, new_s: S) -> bool {
    (!CpuIsValid(old_s, result as Bits64) ==> result == PSCI_INVALID_PARAMETERS)
    && (CpuIsValid(old_s, result as Bits64) && !CpuIsOn(old_s, result as Bits64) ==> result == PSCI_NOT_PRESENT)
    && (CpuIsValid(old_s, result as Bits64) && CpuIsOn(old_s, result as Bits64) && !CallerIsLastCpu(old_s) ==> result == PSCI_DENIED)
    && (CpuIsValid(old_s, result as Bits64) && CpuIsOn(old_s, result as Bits64) && CallerIsLastCpu(old_s) ==> result == PSCI_NOT_SUPPORTED)
    && (CpuIsValid(old_s, result as Bits64) && CpuIsOn(old_s, result as Bits64) && CallerIsLastCpu(old_s) && TrustedOsMigrationInProgress(old_s) ==> result == PSCI_INTERNAL_FAILURE)
    && (CpuIsValid(old_s, result as Bits64) && CpuIsOn(old_s, result as Bits64) && CallerIsLastCpu(old_s) && !TrustedOsMigrationInProgress(old_s) ==> result == PSCI_SUCCESS)
}