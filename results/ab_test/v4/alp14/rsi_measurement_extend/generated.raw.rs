pub open spec fn RSI_MEASUREMENT_EXTEND_spec(
    s: S,
    realm_pre: RmmRealm,
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
    result: RsiCommandReturnCode
) -> bool
{
    let meas_pre = realm_pre.measurements[index as int];
    let new_value = ((((value_0 as int) << 64) | (value_1 as int)) << 128) |
                    (((value_2 as int) << 64) | (value_3 as int)) |
                    ((((value_4 as int) << 64) | (value_5 as int)) << 128) |
                    (((value_6 as int) << 64) | (value_7 as int));
    
    ((index < 1 || index > 4) ==> result == RsiCommandReturnCode::RSI_ERROR_INPUT) &&
    ((size > 64) ==> result == RsiCommandReturnCode::RSI_ERROR_INPUT) &&
    ((!(index < 1 || index > 4) && !(size > 64)) ==>
        (result == RsiCommandReturnCode::RSI_SUCCESS &&
         exists realm: RmmRealm ::
             realm.measurements[index as int] == RemExtend(
                 s,
                 realm_pre.hash_algo,
                 meas_pre,
                 new_value,
                 (size as int) * 8)))
}