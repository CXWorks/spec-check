pub open spec fn rsi_measurement_read_spec(index: UInt64, result: RsiCommandReturnCode, value_0: Bits64, value_1: Bits64, value_2: Bits64, value_3: Bits64, value_4: Bits64, value_5: Bits64, value_6: Bits64, value_7: Bits64, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    ((index > 4) ==> (result == RSI_ERROR_INPUT && new_s == old_s))
    && ((index <= 4) ==> (
        result == RSI_SUCCESS
        && new_s == old_s
        && (realm.hash_algo == HASH_SHA_256 ==> (
            value_0 == RealmMeasurementEncode(old_s, realm.measurements[index as int])[0]
            && value_1 == RealmMeasurementEncode(old_s, realm.measurements[index as int])[1]
            && value_2 == RealmMeasurementEncode(old_s, realm.measurements[index as int])[2]
            && value_3 == RealmMeasurementEncode(old_s, realm.measurements[index as int])[3]
            && value_4 == 0
            && value_5 == 0
            && value_6 == 0
            && value_7 == 0
        ))
        && (realm.hash_algo == HASH_SHA_512 ==> (
            value_0 == RealmMeasurementEncode(old_s, realm.measurements[index as int])[0]
            && value_1 == RealmMeasurementEncode(old_s, realm.measurements[index as int])[1]
            && value_2 == RealmMeasurementEncode(old_s, realm.measurements[index as int])[2]
            && value_3 == RealmMeasurementEncode(old_s, realm.measurements[index as int])[3]
            && value_4 == RealmMeasurementEncode(old_s, realm.measurements[index as int])[4]
            && value_5 == RealmMeasurementEncode(old_s, realm.measurements[index as int])[5]
            && value_6 == RealmMeasurementEncode(old_s, realm.measurements[index as int])[6]
            && value_7 == RealmMeasurementEncode(old_s, realm.measurements[index as int])[7]
        ))
    ))
}
