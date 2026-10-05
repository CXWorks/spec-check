pub open spec fn sensor_axis_name_get__3_7_2_15_spec(result: Int32, flags: UInt32, desc: [AXIS_NAME_DESC], old_s: S, new_s: S) -> bool {
    (!IsValidSensor(old_s, sensor_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidSensorAxis(old_s, sensor_id, axis_id) ==> ResultEqual(result, NOT_FOUND))
    && (!SensorReportsAxisValues(old_s, sensor_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> (flags[5:0] == NumDescriptors(desc) && flags[31:26] == NumRemainingAxisNameDescriptors(old_s, sensor_id, axis_id, flags[5:0]) && flags[25:6] == 0))
    && (old_s == new_s)
}