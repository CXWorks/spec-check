pub open spec fn rsi_measurement_extend_spec(
    result: RsiCommandReturnCode,
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
    old_s: S,
    new_s: S,
) -> bool {
    ((index < 1 || index > 4) ==> result == RSI_ERROR_INPUT) && ((size > 64) ==> result
        == RSI_ERROR_INPUT) && ((index >= 1 && index <= 4 && size <= 64) ==> result == RSI_SUCCESS)
}