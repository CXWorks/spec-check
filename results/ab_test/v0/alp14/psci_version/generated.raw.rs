```verus
pub open spec fn PSCI_VERSION_spec(s: S, input: u64) -> bool {
    input == 0xC4000000 && VersionEqual(s.psci_version, PsciVersion())
}
```