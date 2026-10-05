pub open spec fn sbi_fwft_set_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (IsSuccessfulReturn(result) ==> FeatureValue(feature) == value)
    && (FlagIsSet(flags, LOCK) ==> FeatureIsLocked(feature))
    && (FeatureValue(feature) == value ==> IsSuccessfulReturn(result))
}