pub open spec fn 3.7.2.11_sensor_config_set_spec(sensor_id: UInt32, sensor_config: UInt32, result: Result<int32, ()>, old_s: S, new_s: S) -> bool {
  (result == SCMI_SUCCESS ==> true)
  && (result == SCMI_NOT_FOUND ==> true)
  && (result == SCMI_INVALID_PARAMETERS ==> true)
  && (result == SCMI_NOT_SUPPORTED ==> true)
  && ((!(result == SCMI_SUCCESS) &&
       !(result == SCMI_NOT_FOUND) &&
       !(result == SCMI_INVALID_PARAMETERS) &&
       !(result == SCMI_NOT_SUPPORTED))
    ==> true)
}