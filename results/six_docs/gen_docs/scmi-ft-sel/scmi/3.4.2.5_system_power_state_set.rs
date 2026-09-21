pub open spec fn 3.4.2.5_system_power_state_set_spec(flags: UInt32, system_state: UInt32, result: int, old_s: S, new_s: S) -> bool {
  ((!(result == 0) && (result != SCMI_SUCCESS)) ==> result == SCMI_INVALID_PARAMETERS)
  && ((!(result == 0) && (result != SCMI_SUCCESS)) ==> result == SCMI_NOT_SUPPORTED)
  && ((!(result == 0) && (result != SCMI_SUCCESS)) ==> result == SCMI_DENIED)
  && ((result == 0) ==> result == SCMI_SUCCESS)
}