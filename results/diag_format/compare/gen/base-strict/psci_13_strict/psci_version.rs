pub open spec fn psci_version_spec(version: UInt32, old_s: S, new_s: S) -> bool {
    (!IsImplemented(PSCI_VERSION) ==> ResultEqual(version, NOT_SUPPORTED))
    && (IsImplemented(PSCI_VERSION) ==> (Bits(version, 30, 16) == ImplementedMajorRevision() && Bits(version, 15, 0) == ImplementedMinorRevision() && (forall|f: PsciFunction| (IsInRevision(f, MajorRevision(version), MinorRevisionLowerThan(version)) ==> IsCompatibleInRevision(f, version)))) && (old_s == new_s))
}