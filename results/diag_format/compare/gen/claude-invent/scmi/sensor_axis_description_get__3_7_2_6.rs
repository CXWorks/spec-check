pub open spec fn sensor_axis_description_get__3_7_2_6_spec(old_s: S, new_s: S, sensor_id: UInt32, axis_desc_index: UInt32, status: Int32, num_axis_flags: UInt32) -> bool {
    (!IsValidSensor(old_s, sensor_id) ==> status == NOT_FOUND)
    && ((IsValidSensor(old_s, sensor_id) && !SensorReportsAxisValues(old_s, sensor_id)) ==> status == NOT_SUPPORTED)
    && ((IsValidSensor(old_s, sensor_id) && SensorReportsAxisValues(old_s, sensor_id)) ==> (status == SUCCESS && (num_axis_flags & 0x03FF_FFC0u32) == 0))
    && (new_s == old_s)
}
