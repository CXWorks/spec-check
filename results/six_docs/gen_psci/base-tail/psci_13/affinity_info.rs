pub open spec fn affinity_info_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_INVALID_PARAMETERS ==> AddrIsNonSecure(old_s, 0) == false)
    && (result == PSCI_DISABLED ==> true)
    && (result == PSCI_SUCCESS ==> (result == PSCI_AFFINITY_LEVEL_ON || result == PSCI_AFFINITY_LEVEL_OFF || result == PSCI_AFFINITY_LEVEL_ON_PENDING))
    && (result == PSCI_SUCCESS ==> old_s == new_s)
}