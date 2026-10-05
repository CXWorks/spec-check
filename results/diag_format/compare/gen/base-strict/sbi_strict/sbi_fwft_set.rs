pub open spec fn sbi_fwft_set_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (FeatureValue(new_s, feature) == value)
    && (FeatureValueAtEntry(new_s, feature) == value ==> IsSuccessfulReturn(result))
    && (Bits(flags, 0, 0) == 1 ==> FeatureIsLocked(new_s, feature))
    && (FeatureIsLocked(new_s, feature) && FeatureHasLocalScope(new_s, feature) ==> FeatureValueImmutableUntilHartReset(new_s, feature))
    && (FeatureIsLocked(new_s, feature) && FeatureHasGlobalScope(new_s, feature) ==> FeatureValueImmutableUntilSystemReset(new_s, feature))
}