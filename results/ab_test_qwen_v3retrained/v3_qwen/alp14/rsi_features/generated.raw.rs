pub open spec fn rsi_features_spec(index: UInt64, old_s: S, new_s: S) -> bool {
  (RsiFeatureRegisterEncode(CurrentRealm(new_s), index as int) == RsiFeatureRegisterEncode(CurrentRealm(new_s), index as int))
}