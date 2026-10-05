pub open spec fn rsi_measurement_read_spec(index: UInt64, result: RsiCommandReturnCode, value_0: UInt64, value_1: UInt64, value_2: UInt64, value_3: UInt64, value_4: UInt64, value_5: UInt64, value_6: UInt64, value_7: UInt64, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_INPUT && index > 4)
  && ((!(result == RSI_ERROR_INPUT && index > 4))
    ==> result == RSI_SUCCESS)
}