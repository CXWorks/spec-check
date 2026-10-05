pub open spec fn sensor_axis_description_get__3_7_2_6_spec(sensor_id: UInt32, axis_desc_index: UInt32, status: Int32, num_axis_flags: UInt32, desc: [SENSOR_AXIS_DESC; 8], old_s: S, new_s: S) -> bool {
  (!IsValidSensor(old_s, sensor_id) ==> ResultEqual(status, NOT_FOUND))
  && (!SensorReportsAxisValues(old_s, sensor_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (ResultEqual(status, SUCCESS) ==> desc[0 .. (num_axis_flags & 0x3f) - 1] are valid sensor axis descriptors of sensor_id, starting at index axis_desc_index)
  && (ResultEqual(status, SUCCESS) ==> desc are reported in the normative order of axes of sensor_id)
  && ((IsValidSensor(old_s, sensor_id) &&
       SensorReportsAxisValues(old_s, sensor_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> desc[0 .. (num_axis_flags & 0x3f) - 1] are unchanged)
}