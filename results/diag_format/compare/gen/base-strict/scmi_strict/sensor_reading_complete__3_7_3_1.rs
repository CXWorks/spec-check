pub open spec fn sensor_reading_complete__3_7_3_1_spec(status: Int32, sensor_id: UInt32, readings: SENSOR_READING, old_s: S, new_s: S) -> bool {
    (SensorHasHardwareFault(sensor_id) ==> ResultEqual(status, HARDWARE_ERROR))
    && (ResultEqual(status, SUCCESS) ==> (IsResponseToAsyncSensorReadingGet(sensor_id) && ReadingCount(readings) == SensorAxisCount(sensor_id) && (forall|i: UInt32| i < SensorAxisCount(sensor_id) ==> ReadingAt(readings, i) == SensorAxisReading(sensor_id, i))))
    && (forall|field: Field| FieldIsModified(field, old_s, new_s) ==> false)
}