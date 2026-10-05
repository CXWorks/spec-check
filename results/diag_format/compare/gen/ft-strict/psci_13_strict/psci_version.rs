pub open spec fn psci_version_spec(version: UInt31, old_s: S, new_s: S) -> bool {
  (!IsImplemented(old_s, PSCI_VERSION) ==> NOT_SUPPORTED(version))
  && (Bits(version, 30, 16) == ImplementedMajorRevision(new_s))
  && (Bits(version, 15, 0) == ImplementedMinorRevision(new_s))
  && (forall (f: PsciFunction), (IsInRevision(f, MajorRevision(new_s, version), MinorRevisionLowerThan(new_s, version)) ==> IsCompatibleInRevision(f, version, new_s)))
  && ((!(IsImplemented(old_s, PSCI_VERSION)))
    ==> (Bits(version, 30, 16) == ImplementedMajorRevision(new_s))
    ==> (Bits(version, 15, 0) == ImplementedMinorRevision(new_s)))
}