pub open spec fn sensor_reading_complete__3_7_3_1_spec(sensor_id: UInt32, status: Int32, readings: [SENSOR_READING; 3], old_s: S, new_s: S) -> bool {
  (SensorHasHardwareFault(old_s, sensor_id) ==> ResultEqual(status, HARDWARE_ERROR))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> IsResponseToAsyncSensorReadingGet(new_s, sensor_id))
  && (ResultEqual(status, SUCCESS) ==> ReadingCount(new_s, readings) == SensorAxisCount(new_s, sensor_id))
  && (ResultEqual(status, SUCCESS) ==> (forall (i: UInt32), i < SensorAxisCount(new_s, sensor_id) ==> ReadingAt(new_s, readings, i) == SensorAxisReading(new_s, sensor_id, i)))
  && ((!SensorHasHardwareFault(old_s, sensor_id))
    ==> ResultEqual(status, SUCCESS))
  && (result: Result<(), RmiStatusCode>;
    result.is_Ok()
    ==> ResultEqual(status, SUCCESS))
  && (result: Result<(), RmiStatusCode>;
    result.is_Ok()
    ==> IsResponseToAsyncSensorReadingGet(new_s, sensor_id))
  && (result: Result<(), RmiStatusCode>;
    result.is_Ok()
    ==> ReadingCount(new_s, readings) == SensorAxisCount(new_s, sensor_id))
  && (result: Result<(), RmiStatusCode>;
    result.is_Ok()
    ==> (forall (i: UInt32), i < SensorAxisCount(new_s, sensor_id) ==> ReadingAt(new_s, readings, i) == SensorAxisReading(new_s, sensor_id, i)))
  && ((!(SensorHasHardwareFault(old_s, sensor_id)))
    ==> ResultEqual(status, SUCCESS))
  && (result: Result<(), RmiStatusCode>;
    result.is_Err()
    ==> ResultEqual(status, SUCCESS))
  && (result: Result<(), RmiStatusCode>;
    result.is_Err()
    ==> IsResponseToAsyncSensorReadingGet(new_s, sensor_id))
  && (result: Result<(), RmiStatusCode>;
    result.is_Err()
    ==> ReadingCount(new_s, readings) == SensorAxisCount(new_s, sensor_id))
  && (result: Result<(), RmiStatusCode>;
    result.is_Err()
    ==> (forall (i: UInt32), i < SensorAxisCount(new_s, sensor_id) ==> ReadingAt(new_s, readings, i) == SensorAxisReading(new_s, sensor_id, i)))
}