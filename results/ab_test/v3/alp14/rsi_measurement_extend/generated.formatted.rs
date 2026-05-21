pub open spec fn rsi_measurement_extend_spec(
    result: RsiCommandReturnCode,
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
    old_s: S,
    new_s: S,
) -> bool {
    let realm_pre = old_s;
    let meas_pre = old_s;
    ((index < 1 || index > 4) ==> result == RsiCommandReturnCode::RsiErrorInput) && (size > 64
        ==> result == RsiCommandReturnCode::RsiErrorInput) && (!(index < 1 || index > 4) && !(size
        > 64) ==> (result == RsiCommandReturnCode::RsiOk && new_s.measurements(index as int)
        == RemExtend(
        old_s,
        meas_pre.hash_algo,
        meas_pre,
        ((((value_0 as int) << 64 | (value_1 as int)) << 64 | ((value_2 as int) << 64 | (
        value_3 as int))) << 64 | (((value_4 as int) << 64 | (value_5 as int)) << 64 | ((
        value_6 as int) << 64 | (value_7 as int)))),
        (size * 8) as int,
    )))
}