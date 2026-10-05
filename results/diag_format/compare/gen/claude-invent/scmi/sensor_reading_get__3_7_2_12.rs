pub open spec fn sensor_reading_get__3_7_2_12_spec(sensor_id: u32, flags: u32, status: i32, readings: Seq<SensorReading>, old_s: S, new_s: S) -> bool {
    (!SensorExists(old_s, sensor_id) ==> status == NOT_FOUND)
    && ((SensorExists(old_s, sensor_id) && SensorReadingFlagsInvalid(old_s, sensor_id, flags)) ==> status != SUCCESS)
    && ((SensorExists(old_s, sensor_id) && !SensorEnabled(old_s, sensor_id)) ==> status != SUCCESS)
    && ((SensorExists(old_s, sensor_id) && (flags & 1u32) == 0u32 && SensorHardwareFault(old_s, sensor_id)) ==> status != SUCCESS)
    && (status == INVALID_PARAMETERS ==> (SensorExists(old_s, sensor_id) && SensorReadingFlagsInvalid(old_s, sensor_id, flags)))
    && (status == PROTOCOL_ERROR ==> (SensorExists(old_s, sensor_id) && !SensorEnabled(old_s, sensor_id)))
    && (status == HARDWARE_ERROR ==> SensorHardwareFault(old_s, sensor_id))
    && (status != SUCCESS ==> new_s == old_s)
    && (status == SUCCESS ==> (
        SensorExists(old_s, sensor_id)
        && SensorEnabled(old_s, sensor_id)
        && !SensorReadingFlagsInvalid(old_s, sensor_id, flags)
        && ((flags & 1u32) == 0u32 ==> (
            !SensorHardwareFault(old_s, sensor_id)
            && (SensorHasAxes(old_s, sensor_id) ==> readings.len() == SensorNumAxes(old_s, sensor_id) as nat)
            && (!SensorHasAxes(old_s, sensor_id) ==> readings.len() == 1)
            && (forall|i: int| 0 <= i < readings.len() ==> SensorReadingMatches(old_s, sensor_id, i, readings[i]))
            && new_s == old_s
        ))
        && ((flags & 1u32) == 1u32 ==> AsyncSensorReadPending(new_s, sensor_id))
    ))
}
