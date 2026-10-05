pub open spec fn psci_version_spec(fid: UInt32, result: UInt32, old_s: S, new_s: S) -> bool {
    (!IsFunctionImplemented(PSCI_VERSION) ==> ResultEqual(result, NOT_SUPPORTED))
    && (IsFunctionImplemented(PSCI_VERSION) ==> (
        (((result >> 16u32) & 0x7fffu32) as int) == (ImplementedPsciVersion().major as int)
        && ((result & 0xffffu32) as int) == (ImplementedPsciVersion().minor as int)
        && ((result >> 31u32) & 1u32) == 0u32
    ))
    && new_s == old_s
}
