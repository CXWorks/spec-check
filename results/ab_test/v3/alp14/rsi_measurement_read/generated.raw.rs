```rust
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
    value_7: u64,
) -> bool {
    let realm = CurrentRealm(old_s);
    let meas = realm.measurements[index as int];
    let encoded = RealmMeasurementEncode(meas);
    
    (index > 4 ==> result == RsiCommandReturnCode::RSI_ERROR_INPUT) &&
    (index <= 4 ==> (
        (realm.hash_algo == RmmHashAlgorithm::HASH_SHA_256 ==>
            (value_0 == encoded[0] &&
             value_1 == encoded[1] &&
             value_2 == encoded[2] &&
             value_3 == encoded[3] &&
             value_4 == 0 &&
             value_5 == 0 &&
             value_6 == 0 &&
             value_7 == 0)) &&
        (realm.hash_algo == RmmHashAlgorithm::HASH_SHA_512 ==>
            (value_0 == encoded[0] &&
             value_1 == encoded[1] &&
             value_2 == encoded[2] &&
             value_3 == encoded[3] &&
             value_4 == encoded[4] &&
             value_5 == encoded[5] &&
             value_6 == encoded[6] &&
             value_7 == encoded[7]))
    )) &&
    new_s == old_s
}
```