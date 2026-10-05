pub open spec fn sbi_fwft_set_spec(feature: uint32, value: unsigned long, flags: unsigned long, result: struct sbiret, old_s: S, new_s: S) -> bool {
  (IsSuccessfulReturn(result) ==> FeatureValue(new_s, feature) == value)
  && (IsSuccessfulReturn(result) && FlagIsSet(flags, 0) ==> FeatureIsLocked(new_s, feature))
  && (IsSuccessfulReturn(result) && FeatureValue(old_s, feature) == value ==> IsSuccessfulReturn(result))
  && ((!IsSuccessfulReturn(result))
    ==> FeatureValue(new_s, feature) == FeatureValue(old_s, feature))
  && ((!IsSuccessfulReturn(result))
    ==> FeatureIsLocked(new_s, feature) == FeatureIsLocked(old_s, feature))
  && (result.result != 0
    ==> FeatureValue(new_s, feature) == FeatureValue(old_s, feature))
}