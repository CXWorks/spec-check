pub open spec fn psci_version_spec(fid: UInt64, result: PsciInterfaceVersion, old_s: S, new_s: S) -> bool {
    VersionEqual(result, PsciVersion())
    && new_s == old_s
}