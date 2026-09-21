pub open spec fn base_reset_agent_configuration_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (old_s.cmd_input_flags & 1 != 1 || old_s.cmd_input_flags != 0))
    && (result == SCMI_NOT_FOUND ==> (old_s.cmd_input_agent_id not in old_s.valid_agents))
    && (result == SCMI_NOT_SUPPORTED ==> true)
    && (result == SCMI_DENIED ==> (old_s.cmd_input_agent_id not in old_s.valid_agents || old_s.cmd_input_agent_id != old_s.cmd_input_agent_id))
    && (result == SCMI_SUCCESS ==> (new_s.agent_permissions == old_s.agent_permissions))
    && (result == SCMI_SUCCESS ==> (new_s.platform_resources == old_s.platform_resources))
}