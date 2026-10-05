pub open spec fn sensor_axis_name_get__3_7_2_15_spec(sensor_id: UInt32, axis_id: UInt32, status: Int32, flags: UInt32, desc: [AXIS_NAME_DESC; 1], old_s: S, new_s: S) -> bool {
  (!IsValidSensor(old_s, sensor_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidSensorAxis(old_s, sensor_id, axis_id) ==> ResultEqual(status, NOT_FOUND))
  && (!SensorReportsValuesAlongAxis(old_s, sensor_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 25, 6) == 0)
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 5, 0) == DescriptorCount(new_s, desc))
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 31, 26) == RemainingAxisNameDescriptors(new_s, sensor_id, axis_id, Bits(flags, 5, 0)))
  && (ResultEqual(status, SUCCESS) ==> (forall (i: UInt32), i < Bits(flags, 5, 0) ==> desc[i].axis_id == SensorAxisAtIndex(new_s, sensor_id, axis_id + i)))
  && (ResultEqual(status, SUCCESS) ==> (forall (i: UInt32), i < Bits(flags, 5, 0) ==> IsNullTerminatedUtf8(new_s, desc[i].name, 64)))
  && ((IsValidSensor(old_s, sensor_id) &&
       IsValidSensorAxis(old_s, sensor_id, axis_id) &&
       SensorReportsValuesAlongAxis(old_s, sensor_id))
    ==> ResultEqual(status, SUCCESS))
}