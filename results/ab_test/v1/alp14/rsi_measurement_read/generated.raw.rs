```verus
pub open spec fn RSI_MEASUREMENT_READ_spec(
    s: S,
    index: u64,
    realm: RmmRealm,
) -> (
    result: RsiCommandReturnCode,
    value_0: u64,
    value_1: u64,
    value_2: u64,
    value_3: u64,
    value_4: u64,
    value_5: u64,
    value_6: u64,
    value_7: u64,
) {
    if index > 4 {
        (RSI_ERROR_INPUT, 0, 0, 0, 0, 0, 0, 0, 0)
    } else {
        let meas = realm.measurements[index as int];
        let encoded = RealmMeasurementEncode(meas);
        
        if realm.hash_algo == HASH_SHA_256 {
            (RSI_SUCCESS, encoded[0], encoded[1], encoded[2], encoded[3], 0, 0, 0, 0)
        } else if realm.hash_algo == HASH_SHA_512 {
            (RSI_SUCCESS, encoded[0], encoded[1], encoded[2], encoded[3], encoded[4], encoded[5], encoded[6], encoded[7])
        } else {
            // Default case - should not occur with valid realm state
            (RSI_SUCCESS, 0, 0, 0, 0, 0, 0, 0, 0)
        }
    }
}
```