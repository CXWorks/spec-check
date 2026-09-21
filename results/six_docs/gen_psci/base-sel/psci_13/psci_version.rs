pub open spec fn psci_version_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_SUCCESS || result == PSCI_NOT_SUPPORTED)
}