pub open spec fn node_hw_state_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_NOT_SUPPORTED ==> true)
    && (result == PSCI_INVALID_PARAMETERS ==> true)
    && (result == PSCI_SUCCESS ==> (result == PSCI_SUCCESS))
}