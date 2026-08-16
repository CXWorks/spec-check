pub open spec fn rsi_measurement_extend_spec(result: RsiCommandReturnCode, index: u64, size: u64, value_0: u64, value_1: u64, value_2: u64, value_3: u64, value_4: u64, value_5: u64, value_6: u64, value_7: u64, old_s: S, new_s: S) -> bool {
    ((index < 1 || index > 4) ==> result == RSI_ERROR_INPUT)
    && ((size > 64) ==> result == RSI_ERROR_INPUT)
    && ((index >= 1 && index <= 4 && size <= 64) ==> (
        result == RSI_SUCCESS
        && CurrentRealm(new_s).hash_algo == CurrentRealm(old_s).hash_algo
    ))
}