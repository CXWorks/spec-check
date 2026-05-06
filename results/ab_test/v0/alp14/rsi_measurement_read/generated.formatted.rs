pub open spec fn RSI_MEASUREMENT_READ_spec(
    s: S,
    realm: RmmRealm,
    index: u64,
    result: RsiCommandReturnCode,
    value_0: u64,
    value_1: u64,
    value_2: u64,
    value_3: u64,
    value_4: u64,
    value_5: u64,
    value_6: u64,
    value_7: u64,
) -> bool {
    let meas = realm.measurements[index as int];
    let encoded = RealmMeasurementEncode(s, meas);

    if index > 4 {
        result == RsiCommandReturnCode::RSI_ERROR_INPUT
    } else if realm.hash_algo == RmmHashAlgorithm::HASH_SHA_256 {
        result == RsiCommandReturnCode::RSI_SUCCESS && value_0 == encoded[0] && value_1
            == encoded[1] && value_2 == encoded[2] && value_3 == encoded[3] && value_4 == 0
            && value_5 == 0 && value_6 == 0 && value_7 == 0
    } else if realm.hash_algo == RmmHashAlgorithm::HASH_SHA_512 {
        result == RsiCommandReturnCode::RSI_SUCCESS && value_0 == encoded[0] && value_1
            == encoded[1] && value_2 == encoded[2] && value_3 == encoded[3] && value_4 == encoded[4]
            && value_5 == encoded[5] && value_6 == encoded[6] && value_7 == encoded[7]
    } else {
        false
    }
}