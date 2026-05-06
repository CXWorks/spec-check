pub open spec fn psci_cpu_off_spec(old_s: S, new_s: S) -> bool {
    // PSCI_CPU_OFF command causes a REC exit due to PSCI
    // No failure conditions
    // No success conditions
    // Control does not return to the caller
    // No footprint specified
    true
}