pub open spec fn mem_protect_check_range_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_NOT_SUPPORTED ==> true)
    && (result == PSCI_DENIED ==> true)
    && (result == PSCI_SUCCESS ==> true)
}