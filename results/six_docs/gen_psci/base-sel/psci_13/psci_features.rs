pub open spec fn psci_features_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_NOT_SUPPORTED ==> true)
    && (result != PSCI_NOT_SUPPORTED ==> (result >= 0 && result < 0x8000_0000))
    && (old_s == new_s)
}