pub open spec fn sensor_axis_name_get__3_7_2_15_spec(status: i32, sensor_id: u32, axis_id: u32, flags: u32, desc: Seq<AxisNameDesc>, old_s: S, new_s: S) -> bool {
    (!IsValidSensor(old_s, sensor_id) ==> status == NOT_FOUND)
    && ((IsValidSensor(old_s, sensor_id) && !SensorReportsAxisValues(old_s, sensor_id)) ==> status == NOT_SUPPORTED)
    && ((IsValidSensor(old_s, sensor_id) && SensorReportsAxisValues(old_s, sensor_id) && !IsValidSensorAxis(old_s, sensor_id, axis_id)) ==> status == NOT_FOUND)
    && ((IsValidSensor(old_s, sensor_id) && SensorReportsAxisValues(old_s, sensor_id) && IsValidSensorAxis(old_s, sensor_id, axis_id)) ==> (
        status == SUCCESS
        && ((flags >> 6u32) & 0xFFFFFu32) == 0u32
        && desc.len() == (flags & 0x3Fu32) as int
        && (((flags >> 26u32) & 0x3Fu32) as int)
            == (SensorNumAxes(old_s, sensor_id) as int) - (axis_id as int) - ((flags & 0x3Fu32) as int)
        && (forall|i: int| 0 <= i < desc.len() ==> (
            (desc[i].axis_id as int) == (axis_id as int) + i
            && IsValidSensorAxis(old_s, sensor_id, desc[i].axis_id)
            && desc[i].name == SensorAxisExtendedName(old_s, sensor_id, desc[i].axis_id)
        ))
    ))
    && new_s == old_s
}
