pub open spec fn rsi_measurement_read_spec(result: RsiCommandReturnCode, index: UInt64, old_s: S, new_s: S) -> bool {
    (index > 4 ==> result == RSI_ERROR_INPUT)
}