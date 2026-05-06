pub open spec fn RSI_FEATURES_spec(
    s: S,
    realm: RmmRealm,
    index: u64,
    result: RsiCommandReturnCode,
    value: u64,
) -> bool {
    result == RsiCommandReturnCode::Success && value == RsiFeatureRegisterEncode(
        s,
        realm,
        index as int,
    )
}