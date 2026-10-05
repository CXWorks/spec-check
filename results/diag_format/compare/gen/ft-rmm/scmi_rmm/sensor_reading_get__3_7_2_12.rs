pub open spec fn sensor_reading_get__3_7_2_12_spec(sensor_id: UInt32, flags: Flags, status: Int32, readings: [SENSOR_READING; 1], old_s: S, new_s: S) -> bool {
  (!SensorExists(old_s, sensor_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidReadingFlags(old_s, flags) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!SensorIsEnabled(old_s, sensor_id) ==> ResultEqual(status, PROTOCOL_ERROR))
  && (flags.async == 0 && SensorHasHardwareFault(old_s, sensor_id) ==> ResultEqual(status, HARDWARE_ERROR))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) && flags.async == 0 ==> readings == CurrentSensorReadings(new_s, sensor_id))
  && (ResultEqual(status, SUCCESS) && flags.async == 0 ==> 1 == (SensorIsAxial(new_s, sensor_id) ? SensorNumAxes(new_s, sensor_id) : 1))
  && (ResultEqual(status, SUCCESS) && flags.async == 0 && !SensorCollectsTimestamp(old_s, sensor_id) ==> readings[0].timestamp_low == 0 && readings[0].timestamp_high == 0)
  && (ResultEqual(status, SUCCESS) && flags.async == 1 ==> AsyncReadingRequestEnqueued(new_s, sensor_id))
  && ((SensorExists(old_s, sensor_id) &&
       IsValidReadingFlags(old_s, flags) &&
       SensorIsEnabled(old_s, sensor_id) &&
       !(flags.async == 0 && SensorHasHardwareFault(old_s, sensor_id)))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> CurrentSensorReadings(new_s, sensor_id) == CurrentSensorReadings(old_s, sensor_id))
}