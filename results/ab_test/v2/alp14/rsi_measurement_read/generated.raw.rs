```rust
pub open spec fn rsi_measurement_read_spec(
    result: RsiCommandReturnCode,
    index: u64,
    value_0: u64,
    value_1: u64,
    value_2: u64,
    value_3: u64,
    value_4: u64,
    value_5: u64,
    value_6: u64,
    value_7: u64,
    old_s: S,
    new_s: S
) -> bool {
    let realm = CurrentRealm(old_s);
    let meas = realm.measurements[index as int];
    let encoded = RealmMeasurementEncode(old_s, meas);
    
    // Failure condition: index_bound
    (index > 4 ==> result == RSI_ERROR_INPUT)
    
    // Success condition: sha_256
    && (realm.hash_algo == HASH_SHA_256 && index <= 4 ==>
        result == RSI_SUCCESS
        && value_0 == encoded[0]
        && value_1 == encoded[1]
        && value_2 == encoded[2]
        && value_3 == encoded[3]
        && value_4 == 0
        && value_5 == 0
        && value_6 == 0
        && value_7 == 0)
    
    // Success condition: sha_512
    && (realm.hash_algo == HASH_SHA_512 && index <= 4 ==>
        result == RSI_SUCCESS
        && value_0 == encoded[0]
        && value_1 == encoded[1]
        && value_2 == encoded[2]
        && value_3 == encoded[3]
        && value_4 == encoded[4]
        && value_5 == encoded[5]
        && value_6 == encoded[6]
        && value_7 == encoded[7])
    
    // No state change
    && old_s == new_s
}
```