pub open spec fn RSI_MEASUREMENT_EXTEND_spec(
    s: S,
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
    let realm_pre = CurrentRealm(s);
    let meas_pre = realm_pre.measurements[index as int];
    let realm = CurrentRealm(s);

    (
    // Failure condition: index_bound
    (index < 1 || index > 4) ==> (result == RSI_ERROR_INPUT())) && (
    // Failure condition: size_bound
    (size > 64) ==> (result == RSI_ERROR_INPUT())) && (
    // Success condition: realm_meas
    (!(index < 1 || index > 4) && !(size > 64)) ==> (result == RSI_SUCCESS()
        && realm.measurements[index as int] == RemExtend(
        s,
        realm_pre.hash_algo,
        meas_pre,
        RmmRealmMeasurementConcatenate(
            value_0,
            value_1,
            value_2,
            value_3,
            value_4,
            value_5,
            value_6,
            value_7,
        ),
        (size * 8) as int,
    )))
}