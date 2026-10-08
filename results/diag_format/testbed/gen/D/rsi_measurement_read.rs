pub open spec fn rsi_measurement_read_spec(index: UInt64, result: RsiCommandReturnCode, value_0: Bits64, value_1: Bits64, value_2: Bits64, value_3: Bits64, value_4: Bits64, value_5: Bits64, value_6: Bits64, value_7: Bits64, old_s: S, new_s: S) -> bool {
    (index > 4 ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> (forall i: 0..7 | Bits64(value_i) == Bits64(RimExtendData(old_s, CurrentRealm(old_s), 0, old_s.mem[old_s.mem.len - 1], RMI_MEASURE_CONTENT)[i])))
}