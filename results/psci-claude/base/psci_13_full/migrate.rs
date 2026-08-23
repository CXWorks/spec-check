pub open spec fn migrate_spec(result: Int64, old_s: S, new_s: S, target_cpu: Bits64) -> bool {
    (!CpuIsValid(old_s, target_cpu) ==> result == PSCI_INVALID_PARAMETERS)
    && (!CpuIsOn(old_s, target_cpu) ==> result == PSCI_INTERNAL_FAILURE)
    && (TrustedOsMigrationInProgress(old_s) ==> result == PSCI_DENIED)
    && ((CpuIsValid(old_s, target_cpu)
         && CpuIsOn(old_s, target_cpu)
         && !TrustedOsMigrationInProgress(old_s))
        ==> result == PSCI_SUCCESS)
}