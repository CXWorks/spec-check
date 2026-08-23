pub open spec fn cpu_off_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (result as int == PSCI_DENIED ==> new_s == old_s)
    && (new_s != old_s ==> result as int != PSCI_DENIED)
}