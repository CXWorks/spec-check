pub open spec fn sensor_config_set__3_7_2_11_spec(sensor_id: UInt32, sec: UInt16, exponent: Int5, round_auto: Bool, round_up: Bool, timestamp: Bool, sensor_state: Bool, status: Int32, sensor_config: [UInt32; 2], old_s: S, new_s: S) -> bool {
  (!SensorExists(old_s, sensor_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidSensorConfig(old_s, sensor_id, sensor_config) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!SensorSupportsConfig(old_s, sensor_id, sensor_config) ==> ResultEqual(status, NOT_SUPPORTED))
  && (ResultEqual(status, SUCCESS) ==> SensorAt(new_s, sensor_id).enabled == (Bits(sensor_config, 0, 0) == 1))
  && (ResultEqual(status, SUCCESS) ==> SensorAt(new_s, sensor_id).timestamp_enabled == (Bits(sensor_config, 1, 1) == 1))
  && (ResultEqual(status, SUCCESS) ==> Bits(sensor_config, 31, 11) == 0 ==> UpdateIntervalUnchanged(new_s, sensor_id))
  && (ResultEqual(status, SUCCESS) ==> (Bits(sensor_config, 31, 11) != 0 && Bits(sensor_config, 10, 10) == 1) ==> SensorAt(new_s, sensor_id).update_interval == ClosestSupportedInterval(new_s, sensor_id, RequestedInterval(Bits(sensor_config, 31, 16), Bits(sensor_config, 15, 11)))))
  && (ResultEqual(status, SUCCESS) ==> (Bits(sensor_config, 31, 11) != 0 && Bits(sensor_config, 10, 10) == 0 && Bits(sensor_config, 9, 9) == 1) ==> SensorAt(new_s, sensor_id).update_interval == RoundUpSupportedInterval(new_s, sensor_id, RequestedInterval(Bits(sensor_config, 31, 16), Bits(sensor_config, 15, 11)))))
  && (ResultEqual(status, SUCCESS) ==> (Bits(sensor_config, 31, 11) != 0 && Bits(sensor_config, 10, 10) == 0 && Bits(sensor_config, 9, 9) == 0) ==> SensorAt(new_s, sensor_id).update_interval == RoundDownSupportedInterval(new_s, sensor_id, RequestedInterval(Bits(sensor_config, 31, 16), Bits(sensor_config, 15, 11)))))
  && ((!(SensorExists(old_s, sensor_id)) &&
       IsValidSensorConfig(old_s, sensor_id, sensor_config) &&
       SensorSupportsConfig(old_s, sensor_id, sensor_config))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> SensorAt(new_s, sensor_id).enabled == SensorAt(old_s, sensor_id).enabled)
  && (result != SUCCESS
    ==> SensorAt(new_s, sensor_id).timestamp_enabled == SensorAt(old_s, sensor_id).timestamp_enabled)
  && (result != SUCCESS
    ==> SensorAt(new_s, sensor_id).update_interval == SensorAt(old_s, sensor_id).update_interval)
}