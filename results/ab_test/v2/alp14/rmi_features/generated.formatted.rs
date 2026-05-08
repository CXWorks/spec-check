pub open spec fn rmi_features_spec(
    result: RmiCommandReturnCode,
    value: u64,
    old_s: S,
    new_s: S,
    index: u64,
) -> bool {
    result == RmiCommandReturnCode::Success && value == RmiFeatureRegisterEncode(
        old_s,
        index as int,
    )
}