pub open spec fn rsi_measurement_extend_spec(
    result: RsiCommandReturnCode,
    old_s: S,
    new_s: S,
    fid: UInt64,
    index: UInt64,
    size: UInt64,
    value_0: Bits64,
    value_1: Bits64,
    value_2: Bits64,
    value_3: Bits64,
    value_4: Bits64,
    value_5: Bits64,
    value_6: Bits64,
    value_7: Bits64,
) -> bool {
    ((index < 1 || index > 4) ==> result == RSI_ERROR_INPUT)
    && (size > 64 ==> result == RSI_ERROR_INPUT)
    && ((index >= 1 && index <= 4 && size <= 64) ==> (
        result == RSI_SUCCESS
        && CurrentRealm(new_s).measurements[index as usize]
            == RemExtend(
                old_s,
                CurrentRealm(old_s).hash_algo,
                CurrentRealm(old_s).measurements[index as usize],
                [value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7],
                size as int,
            )
    ))
}