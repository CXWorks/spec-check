pub open spec fn rmi_features_spec(
    index: UInt64,
    result: Result<(), RmiStatusCode>,
    value: Bits64,
    old_s: S,
    new_s: S,
) -> bool {
    (index != 0 ==> result.is_Ok() && value == 0) && (index == 0 ==> result.is_Ok()) && new_s
        == old_s
}