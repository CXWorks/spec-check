pub open spec fn rsi_measurement_extend_spec(index: UInt64, size: UInt64, value_0: Bits64, value_1: Bits64, value_2: Bits64, value_3: Bits64, value_4: Bits64, value_5: Bits64, value_6: Bits64, value_7: Bits64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (index < 1 || index > 4 ==> result == RSI_ERROR_INPUT)
  && (size > 64 ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> CurrentRealm(new_s).measurements[index] == RemExtend(new_s, CurrentRealm(new_s).hash_algo, CurrentRealm(new_s).measurements[index], [value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7], (RMM_REALM_MEASUREMENT_WIDTH-1):0, size))
  && ((!(index < 1 || index > 4) &&
       !(size > 64))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> CurrentRealm(new_s).measurements[index] == CurrentRealm(old_s).measurements[index])
}