pub open spec fn sensor_axis_name_get__3_7_2_15_spec(sensor_id: UInt32, axis_id: UInt32, status: Int32, flags: UInt32, desc: [AXIS_NAME_DESC; 1], old_s: S, new_s: S) -> bool {
  (!IsValidSensor(old_s, sensor_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidSensorAxis(old_s, sensor_id, axis_id) ==> ResultEqual(status, NOT_FOUND))
  && (!SensorReportsAxisValues(old_s, sensor_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> flags[5:0] == NumDescriptors(new_s, desc))
  && (ResultEqual(status, SUCCESS) ==> flags[31:26] == NumRemainingAxisNameDescriptors(new_s, sensor_id, axis_id, flags[5:0]))
  && (ResultEqual(status, SUCCESS) ==> flags[25:6] == 0)
  && ((IsValidSensor(old_s, sensor_id) &&
       IsValidSensorAxis(old_s, sensor_id, axis_id) &&
       SensorReportsAxisValues(old_s, sensor_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> flags[5:0] == 0)
  && (result != SUCCESS
    ==> flags[31:26] == 0)
  && (result != SUCCESS
    ==> flags[25:6] == 0)
}