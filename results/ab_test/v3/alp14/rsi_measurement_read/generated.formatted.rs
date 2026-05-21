pub open spec fn rsi_measurement_read_spec(
    result: RsiCommandReturnCode,
    old_s: S,
    new_s: S,
    index: u64,
    value_0: u64,
    value_1: u64,
    value_2: u64,
    value_3: u64,
    value_4: u64,
    value_5: u64,
    value_6: u64,
    value_7: u64,
) -> bool {
    let realm = old_s.current_realm;
    let meas = realm.measurements[index as int];
    (index > 4 ==> result == RSI_ERROR_INPUT) && (index <= 4 && realm.hash_algo == HASH_SHA_256
        ==> (result == RSI_OK && value_0 == RealmMeasurementEncode(meas)[0] && value_1
        == RealmMeasurementEncode(meas)[1] && value_2 == RealmMeasurementEncode(meas)[2] && value_3
        == RealmMeasurementEncode(meas)[3] && value_4 == 0 && value_5 == 0 && value_6 == 0
        && value_7 == 0 && new_s == old_s)) && (index <= 4 && realm.hash_algo == HASH_SHA_512 ==> (
    result == RSI_OK && value_0 == RealmMeasurementEncode(meas)[0] && value_1
        == RealmMeasurementEncode(meas)[1] && value_2 == RealmMeasurementEncode(meas)[2] && value_3
        == RealmMeasurementEncode(meas)[3] && value_4 == RealmMeasurementEncode(meas)[4] && value_5
        == RealmMeasurementEncode(meas)[5] && value_6 == RealmMeasurementEncode(meas)[6] && value_7
        == RealmMeasurementEncode(meas)[7] && new_s == old_s))
}