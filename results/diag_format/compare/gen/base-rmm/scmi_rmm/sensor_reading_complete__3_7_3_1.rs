pub open spec fn sensor_reading_complete__3_7_3_1_spec(result: int32, sensor_id: uint32, readings: SENSOR_READING, old_s: S, new_s: S) -> bool {
    (SensorHasHardwareFault(sensor_id) ==> ResultEqual(result, HARDWARE_ERROR))
    && (Length(readings) == NumSensorAxes(sensor_id))
    && (ReadingsReportedInAxisOrder(readings, sensor_id))
}