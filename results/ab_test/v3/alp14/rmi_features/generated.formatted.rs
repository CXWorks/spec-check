pub open spec fn rmi_features_spec(
    result: RmiCommandReturnCode,
    value: u64,
    index: u64,
    old_s: S,
    new_s: S,
) -> bool {
    value == RmiFeatureRegisterEncode(old_s, index as int)
}