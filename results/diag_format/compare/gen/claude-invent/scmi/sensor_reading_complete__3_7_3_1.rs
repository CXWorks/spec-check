pub open spec fn sensor_reading_complete__3_7_3_1_spec(status: i32, sensor_id: u32, readings: Seq<SENSOR_READING>, old_s: S, new_s: S) -> bool {
    (SensorHasHardwareFault(old_s, sensor_id) ==> status == HARDWARE_ERROR)
    && (status == SUCCESS ==> (
        !SensorHasHardwareFault(old_s, sensor_id)
        && readings.len() == SensorNumAxes(old_s, sensor_id)
        && (forall|i: int| 0 <= i < readings.len() ==> readings[i] == SensorAxisReading(old_s, sensor_id, i))
    ))
    && new_s == old_s
}
