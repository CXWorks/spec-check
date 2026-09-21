pub open spec fn 3.2.2.13_base_reset_agent_configuration_spec(agent_id: UInt32, flags: UInt32, result: Result<(), int>, old_s: S, new_s: S) -> bool {
  (result == SCMI_INVALID_PARAMETERS && (flags & 1) != 0)
  && ((!(result == SCMI_INVALID_PARAMETERS && (flags & 1) != 0))
    ==> result == SCMI_SUCCESS)
}