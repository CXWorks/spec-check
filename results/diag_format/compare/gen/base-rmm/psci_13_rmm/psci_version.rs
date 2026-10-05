pub open spec fn psci_version_spec(result: u64, old_s: S, new_s: S) -> bool {
    (!IsFunctionImplemented(PSCI_VERSION) ==> ResultEqual(result, NOT_SUPPORTED))
    && (IsFunctionImplemented(PSCI_VERSION) ==> (result[31] == 0 && result[30:16] == ImplementedPsciVersion().major && result[15:0] == ImplementedPsciVersion().minor))
    && (old_s == new_s)
}