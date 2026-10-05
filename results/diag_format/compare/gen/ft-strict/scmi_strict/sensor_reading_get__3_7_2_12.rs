pub open spec fn sensor_reading_get__3_7_2_12_spec(sensor_id: UInt32, flags: UInt32, status: Int32, readings: [SENSOR_READING; 4], result: Result, old_s: S, new_s: S) -> bool {
  (!SensorExists(old_s, sensor_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidReadingFlags(old_s, flags) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!SensorIsEnabled(old_s, sensor_id) ==> ResultEqual(status, PROTOCOL_ERROR))
  && (Bits(flags, 0, 0) == 0 && SensorHardwareFault(old_s, sensor_id) ==> ResultEqual(status, HARDWARE_ERROR))
  && (result == SUCCESS ==> (Bits(flags, 0, 0) == 0 ==> ReadingsMatchCurrentSensorValue(new_s, readings, sensor_id)))
  && (result == SUCCESS ==> (Bits(flags, 0, 0) == 1 ==> SensorReadingCompleteEnqueued(new_s, sensor_id)))
  && (result == SUCCESS ==> SensorIsScalar(old_s, sensor_id) ==> ReadingCount(new_s, readings) == 1)
  && (result == SUCCESS ==> !SensorIsScalar(old_s, sensor_id) ==> ReadingCount(new_s, readings) == SensorAxisCount(new_s, sensor_id))
  && (result == SUCCESS ==> !SensorIsScalar(old_s, sensor_id) ==> ReadingsInAxisOrder(new_s, readings, sensor_id))
  && (result == SUCCESS ==> (forall i: UInt32, (i < ReadingCount(new_s, readings) && !TimestampCollected(new_s, sensor_id)) ==> (readings[i].timestamp_low == 0 && readings[i].timestamp_high == 0)))
  && ((SensorExists(old_s, sensor_id) &&
       IsValidReadingFlags(old_s, flags) &&
       SensorIsEnabled(old_s, sensor_id) &&
       !(Bits(flags, 0, 0) == 0 && SensorHardwareFault(old_s, sensor_id)))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> ReadingsMatchCurrentSensorValue(new_s, readings, sensor_id))
  && (result != SUCCESS
    ==> SensorReadingCompleteEnqueued(new_s, sensor_id))
  && (result != SUCCESS
    ==> ReadingCount(new_s, readings) == 1)
  && (result != SUCCESS
    ==> ReadingCount(new_s, readings) == SensorAxisCount(new_s, sensor_id))
  && (result != SUCCESS
    ==> ReadingsInAxisOrder(new_s, readings, sensor_id))
  && (result != SUCCESS
    ==> (forall i: UInt32, (i < ReadingCount(new_s, readings) && !TimestampCollected(new_s, sensor_id)) ==> (readings[i].timestamp_low == 0 && readings[i].timestamp_high == 0)))
  && (SensorAt(new_s, sensor_id).pending_reading_complete == SensorAt(old_s, sensor_id).pending_reading_complete)
}