pub open spec fn rsi_measurement_read_spec(index: UInt64, result: RsiCommandReturnCode, value_0: Bits64, value_1: Bits64, value_2: Bits64, value_3: Bits64, value_4: Bits64, value_5: Bits64, value_6: Bits64, value_7: Bits64, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_INPUT ==> index > 4)
  && ((result == RSI_SUCCESS)
    ==> CurrentRealm(new_s).hash_algo == HASH_SHA_256
    ==> value_0 == CurrentRealm(new_s).measurements[index][0]
    ==> value_1 == CurrentRealm(new_s).measurements[index][1]
    ==> value_2 == CurrentRealm(new_s).measurements[index][2]
    ==> value_3 == CurrentRealm(new_s).measurements[index][3]
    ==> value_4 == 0
    ==> value_5 == 0
    ==> value_6 == 0
    ==> value_7 == 0)
  && ((result == RSI_SUCCESS)
    ==> CurrentRealm(new_s).hash_algo == HASH_SHA_512
    ==> value_0 == CurrentRealm(new_s).measurements[index][0]
    ==> value_1 == CurrentRealm(new_s).measurements[index][1]
    ==> value_2 == CurrentRealm(new_s).measurements[index][2]
    ==> value_3 == CurrentRealm(new_s).measurements[index][3]
    ==> value_4 == CurrentRealm(new_s).measurements[index][4]
    ==> value_5 == CurrentRealm(new_s).measurements[index][5]
    ==> value_6 == CurrentRealm(new_s).measurements[index][6]
    ==> value_7 == CurrentRealm(new_s).measurements[index][7])
  && ((!(result == RSI_ERROR_INPUT))
    ==> result == RSI_SUCCESS)
}