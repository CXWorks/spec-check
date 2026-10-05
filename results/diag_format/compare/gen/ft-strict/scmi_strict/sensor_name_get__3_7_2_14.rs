pub open spec fn sensor_name_get__3_7_2_14_spec(sensor_id: UInt32, status: Int32, flags: UInt32, name: [UInt8; 64], old_s: S, new_s: S) -> bool {
  (!SensorExists(old_s, sensor_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> flags == 0)
  && (ResultEqual(status, SUCCESS) ==> IsValidSensorExtendedName(new_s, sensor_id, name))
  && (ResultEqual(status, SUCCESS) ==> IsNullTerminatedUtf8String(name, 64))
  && ((SensorExists(old_s, sensor_id))
    ==> ResultEqual(status, SUCCESS))
}