pub open spec fn sensor_axis_description_get__3_7_2_6_spec(status: Int32, num_axis_flags: UInt32, desc: [SENSOR_AXIS_DESC], old_s: S, new_s: S) -> bool {
    (!IsValidSensor(sensor_id(old_s)) ==> ResultEqual(status, NOT_FOUND))
    && (!SensorReportsAxisValues(sensor_id(old_s)) ==> ResultEqual(status, NOT_SUPPORTED))
    && (ResultEqual(status, SUCCESS) ==> (
        (num_axis_flags & 0x3F) == 0
        && (num_axis_flags & 0x3F) > 0
        && (num_axis_flags & 0x3F) <= 16
        && (forall i: int | 0 <= i && i < (num_axis_flags & 0x3F) as int ==> ValidSensorAxisDescriptor(desc[i], sensor_id(old_s)))
        && (forall i: int | 0 <= i && i < (num_axis_flags & 0x3F) as int ==> SensorAxisDescriptorOrder(desc[i], sensor_id(old_s)))
    ))
    && (old_s == new_s)
}