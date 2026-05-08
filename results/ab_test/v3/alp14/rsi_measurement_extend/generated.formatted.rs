pub open spec fn RSI_MEASUREMENT_EXTEND_spec(
    old_s: S,
    new_s: S,
    index: u64,
    size: u64,
    value_0: u64,
    value_1: u64,
    value_2: u64,
    value_3: u64,
    value_4: u64,
    value_5: u64,
    value_6: u64,
    value_7: u64,
    result: RsiCommandReturnCode,
) -> bool {
    let realm_pre = old_s.current_realm();
    let realm = new_s.current_realm();
    let meas_pre = realm_pre.measurements[index as int];
    let measurement_value = ToBits64(index as int);

    (
    // Failure: index_bound
    (index < 1 || index > 4) ==> result == RSI_ERROR_INPUT) && (
    // Failure: size_bound
    (size > 64) ==> result == RSI_ERROR_INPUT) && (
    // Success: realm_meas
    (!(index < 1 || index > 4) && !(size > 64)) ==> (result == RSI_OK
        && realm.measurements[index as int] == RemExtend(
        old_s,
        realm_pre.hash_algo,
        meas_pre,
        RmmRealmMeasurement {
            value_0: value_0,
            value_1: value_1,
            value_2: value_2,
            value_3: value_3,
            value_4: value_4,
            value_5: value_5,
            value_6: value_6,
            value_7: value_7,
        },
        (size * 8) as int,
    )))
}