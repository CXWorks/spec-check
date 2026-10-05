pub open spec fn sensor_axis_name_get__3_7_2_15_spec(result: Int32, flags: UInt32, desc: [AXIS_NAME_DESC; 0], old_s: S, new_s: S) -> bool {
    (!IsValidSensor(old_s, sensor_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidSensorAxis(old_s, sensor_id(old_s), axis_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!SensorReportsValuesAlongAxis(old_s, sensor_id(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> (Bits64(flags, 25, 6) == 0))
    && (ResultEqual(result, SUCCESS) ==> (Bits64(flags, 5, 0) == DescriptorCount(desc)))
    && (ResultEqual(result, SUCCESS) ==> (Bits64(flags, 31, 26) == RemainingAxisNameDescriptors(old_s, sensor_id(old_s), axis_id(old_s), Bits64(flags, 5, 0))))
    && (ResultEqual(result, SUCCESS) ==> (forall i: UInt32 | i < Bits64(flags, 5, 0) ==> desc[i].axis_id == SensorAxisAtIndex(old_s, sensor_id(old_s), axis_id(old_s) + i)))
    && (ResultEqual(result, SUCCESS) ==> (forall i: UInt32 | i < Bits64(flags, 5, 0) ==> IsNullTerminatedUtf8(desc[i].name, 64)))
}