pub open spec fn psci_set_suspend_mode_spec(result: int, mode: Bits64, old_s: S, new_s: S) -> bool {
    (!(mode == 0 || mode == 1) ==> result == PSCI_INVALID_PARAMETERS)
    && ((mode == 0 || mode == 1) ==> (result == PSCI_SUCCESS || result == PSCI_DENIED || result == PSCI_NOT_SUPPORTED))
    && (result != PSCI_SUCCESS ==> new_s == old_s)
}