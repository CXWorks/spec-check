pub open spec fn PSCI_VERSION_spec(old_s: S, new_s: S, result: PsciInterfaceVersion) -> bool {
    VersionEqual(result, PsciVersion())
}