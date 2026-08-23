pub open spec fn migrate_spec(result: int, target_cpu: Bits64, old_s: S, new_s: S) -> bool {
    (!CpuIsValid(old_s, target_cpu) ==> result == PSCI_INVALID_PARAMETERS)
    && (CpuIsValid(old_s, target_cpu) && !CpuIsOn(old_s, target_cpu) ==> result == PSCI_INTERNAL_FAILURE)
    && (CpuIsValid(old_s, target_cpu) && !CallerIsLastCpu(old_s) ==> result == PSCI_NOT_PRESENT)
    && (CpuIsValid(old_s, target_cpu) && TrustedOsMigrationInProgress(old_s) ==> result == PSCI_DENIED)
    && (result != PSCI_SUCCESS ==> new_s == old_s)
}