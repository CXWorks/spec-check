pub open spec fn cpu_off_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (TrustedOsMigrationInProgress(old_s) ==> result as int == PSCI_DENIED)
    && (!TrustedOsMigrationInProgress(old_s) ==> result as int == PSCI_DENIED || true)
}