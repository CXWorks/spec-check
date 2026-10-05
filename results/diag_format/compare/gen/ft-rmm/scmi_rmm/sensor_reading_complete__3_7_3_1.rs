pub open spec fn sensor_reading_complete__3_7_3_1_spec(message_id: UInt, protocol_id: UInt, status: int32, sensor_id: uint32, readings: SENSOR_READING, old_s: S, new_s: S) -> bool {
  (SensorHasHardwareFault(old_s, sensor_id) ==> ResultEqual(status, HARDWARE_ERROR))
  && (Length(readings) == NumSensorAxes(old_s, sensor_id))
  && (ReadingsReportedInAxisOrder(readings, sensor_id))
  && ((!SensorHasHardwareFault(old_s, sensor_id))
    ==> status != HARDWARE_ERROR)
}