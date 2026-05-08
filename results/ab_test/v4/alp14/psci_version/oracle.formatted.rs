pub open spec fn psci_version_spec(result: PsciInterfaceVersion, old_s: S, new_s: S) -> bool {
    (result == PSCI_SUCCESS ==> VersionEqual(result, PsciVersion(new_s)))
}