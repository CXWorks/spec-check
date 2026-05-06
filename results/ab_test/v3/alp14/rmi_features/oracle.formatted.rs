pub open spec fn rmi_features_spec(
    index: UInt64,
    result: Result<(), RmiStatusCode>,
    value: Bits64,
    old_s: S,
    new_s: S,
) -> bool {
    (result.is_Ok() ==> value == RmiFeatureRegisterEncode(new_s, index as int))
}