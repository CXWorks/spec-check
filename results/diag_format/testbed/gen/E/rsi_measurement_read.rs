pub open spec fn rsi_measurement_read_spec(index: UInt64, result: RsiCommandReturnCode, value_0: Bits64, value_1: Bits64, value_2: Bits64, value_3: Bits64, value_4: Bits64, value_5: Bits64, value_6: Bits64, value_7: Bits64, old_s: S, new_s: S) -> bool {
  (index > 4 ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> value_0 == CurrentRealm(new_s).measurements[index as int][0])
  && (result == RSI_SUCCESS ==> value_1 == CurrentRealm(new_s).measurements[index as int][1])
  && (result == RSI_SUCCESS ==> value_2 == CurrentRealm(new_s).measurements[index as int][2])
  && (result == RSI_SUCCESS ==> value_3 == CurrentRealm(new_s).measurements[index as int][3])
  && (result == RSI_SUCCESS ==> value_4 == CurrentRealm(new_s).measurements[index as int][4])
  && (result == RSI_SUCCESS ==> value_5 == CurrentRealm(new_s).measurements[index as int][5])
  && (result == RSI_SUCCESS ==> value_6 == CurrentRealm(new_s).measurements[index as int][6])
  && (result == RSI_SUCCESS ==> value_7 == CurrentRealm(new_s).measurements[index as int][7])
  && ((!(index > 4))
    ==> result == RSI_SUCCESS)
}