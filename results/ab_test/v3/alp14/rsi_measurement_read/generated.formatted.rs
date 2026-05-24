pub open spec fn rsi_measurement_read_spec(result: RsiCommandReturnCode, old_s: S, new_s: S, index: u64, value_0: u64, value_1: u64, value_2: u64, value_3: u64, value_4: u64, value_5: u64, value_6: u64, value_7: u64) -> bool {
    (index > 4 ==> result == RsiCommandReturnCode::RsiErrorInput)
    && ((index <= 4 && CurrentRealm(old_s).hash_algo == RmmHashAlgorithm::HashSha256) ==> (
        result.is_Ok()
        && value_0 == RealmMeasurementEncode(old_s.measurements[index as int])[0]
        && value_1 == RealmMeasurementEncode(old_s.measurements[index as int])[1]
        && value_2 == RealmMeasurementEncode(old_s.measurements[index as int])[2]
        && value_3 == RealmMeasurementEncode(old_s.measurements[index as int])[3]
        && value_4 == 0
        && value_5 == 0
        && value_6 == 0
        && value_7 == 0
        && new_s == old_s
    ))
    && ((index <= 4 && CurrentRealm(old_s).hash_algo == RmmHashAlgorithm::HashSha512) ==> (
        result.is_Ok()
        && value_0 == RealmMeasurementEncode(old_s.measurements[index as int])[0]
        && value_1 == RealmMeasurementEncode(old_s.measurements[index as int])[1]
        && value_2 == RealmMeasurementEncode(old_s.measurements[index as int])[2]
        && value_3 == RealmMeasurementEncode(old_s.measurements[index as int])[3]
        && value_4 == RealmMeasurementEncode(old_s.measurements[index as int])[4]
        && value_5 == RealmMeasurementEncode(old_s.measurements[index as int])[5]
        && value_6 == RealmMeasurementEncode(old_s.measurements[index as int])[6]
        && value_7 == RealmMeasurementEncode(old_s.measurements[index as int])[7]
        && new_s == old_s
    ))
}