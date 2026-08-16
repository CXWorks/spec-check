pub open spec fn rsi_features_spec(
    index: UInt64,
    result: RsiCommandReturnCode,
    value: Bits64,
    old_s: S,
    new_s: S,
) -> bool {
    (result == RSI_SUCCESS ==> value == 0)
}