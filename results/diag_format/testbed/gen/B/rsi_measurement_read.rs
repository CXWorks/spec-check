pub open spec fn rsi_measurement_read_spec(index: UInt64, result: RsiCommandReturnCode, value_0: Bits64, value_1: Bits64, value_2: Bits64, value_3: Bits64, value_4: Bits64, value_5: Bits64, value_6: Bits64, value_7: Bits64, old_s: S, new_s: S) -> bool {
    (index > 4 ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> (
        let realm = CurrentRealm(old_s);
        let meas = realm.measurements[index as int];
        let encoded = RealmMeasurementEncode(meas);
        (realm.hash_algo == HASH_SHA_256 ==> (
            value_0 == Element(encoded, 0)
            && value_1 == Element(encoded, 1)
            && value_2 == Element(encoded, 2)
            && value_3 == Element(encoded, 3)
            && value_4 == 0
            && value_5 == 0
            && value_6 == 0
            && value_7 == 0
        ))
        && (realm.hash_algo == HASH_SHA_512 ==> (
            value_0 == Element(encoded, 0)
            && value_1 == Element(encoded, 1)
            && value_2 == Element(encoded, 2)
            && value_3 == Element(encoded, 3)
            && value_4 == Element(encoded, 4)
            && value_5 == Element(encoded, 5)
            && value_6 == Element(encoded, 6)
            && value_7 == Element(encoded, 7)
        ))
    ))
}