pub open spec fn sensor_config_get__3_7_2_10_spec(sensor_id: UInt32, status: Int32, sensor_config: UInt32, old_s: S, new_s: S) -> bool {
  (!SensorExists(old_s, sensor_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> SensorUpdateIntervalSupported(old_s, sensor_id) ==> Bits(sensor_config, 31, 16) == SensorUpdateIntervalSec(old_s, sensor_id))
  && (ResultEqual(status, SUCCESS) ==> SensorUpdateIntervalSupported(old_s, sensor_id) ==> Bits(sensor_config, 15, 11) == SensorUpdateIntervalExponent(old_s, sensor_id))
  && (ResultEqual(status, SUCCESS) ==> !SensorUpdateIntervalSupported(old_s, sensor_id) ==> Bits(sensor_config, 31, 11) == 0)
  && (ResultEqual(status, SUCCESS) ==> SensorIsTimestamped(old_s, sensor_id) ==> Bits(sensor_config, 1, 1) == 1)
  && (ResultEqual(status, SUCCESS) ==> !SensorIsTimestamped(old_s, sensor_id) ==> Bits(sensor_config, 1, 1) == 0)
  && (ResultEqual(status, SUCCESS) ==> SensorIsEnabled(old_s, sensor_id) ==> Bits(sensor_config, 0, 0) == 1)
  && (ResultEqual(status, SUCCESS) ==> !SensorIsEnabled(old_s, sensor_id) ==> Bits(sensor_config, 0, 0) == 0)
  && ((SensorExists(old_s, sensor_id))
    ==> ResultEqual(status, SUCCESS))
}