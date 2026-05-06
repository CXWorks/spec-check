pub open spec fn RSI_MEASUREMENT_READ_spec(
    old_s: S,
    new_s: S,
    index: u64,
    result: RsiCommandReturnCode,
    value_0: u64,
    value_1: u64,
    value_2: u64,
    value_3: u64,
    value_4: u64,
    value_5: u64,
    value_6: u64,
    value_7: u64
) -> bool {
    let realm = CurrentRealm(old_s);
    let meas = realm.measurements[index as int];
    
    (
        (index as int > 4 ==> result == RsiCommandReturnCode::RSI_ERROR_INPUT)
        &&
        (index as int <= 4 && realm.hash_algo == RmmHashAlgorithm::HASH_SHA_256 ==>
            (value_0 == RealmMeasurementEncode(old_s, meas)[0]
             && value_1 == RealmMeasurementEncode(old_s, meas)[1]
             && value_2 == RealmMeasurementEncode(old_s, meas)[2]
             && value_3 == RealmMeasurementEncode(old_s, meas)[3]
             && value_4 == 0
             && value_5 == 0
             && value_6 == 0
             && value_7 == 0))
        &&
        (index as int <= 4 && realm.hash_algo == RmmHashAlgorithm::HASH_SHA_512 ==>
            (value_0 == RealmMeasurementEncode(old_s, meas)[0]
             && value_1 == RealmMeasurementEncode(old_s, meas)[1]
             && value_2 == RealmMeasurementEncode(old_s, meas)[2]
             && value_3 == RealmMeasurementEncode(old_s, meas)[3]
             && value_4 == RealmMeasurementEncode(old_s, meas)[4]
             && value_5 == RealmMeasurementEncode(old_s, meas)[5]
             && value_6 == RealmMeasurementEncode(old_s, meas)[6]
             && value_7 == RealmMeasurementEncode(old_s, meas)[7]))
        &&
        old_s == new_s
    )
}