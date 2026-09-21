pub open spec fn 3.7.2.14_sensor_name_get_spec(sensor_id: UInt32, status: int32, flags: UInt32, name: [uint8; 64], old_s: S, new_s: S) -> bool {
  (status == SCMI_SUCCESS ==> flags == 0)
}