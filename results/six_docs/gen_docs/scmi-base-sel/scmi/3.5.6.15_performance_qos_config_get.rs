pub open spec fn 3.5.6.15_performance_qos_config_get_spec(result: int, qos_value: UInt32, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (old_s.cmd_input_capability & (old_s.cmd_input_capability - 1) != 0))
    && (result == SCMI_NOT_FOUND ==> (old_s.cmd_input_domain_id != old_s.cmd_input_domain_id || (old_s.cmd_input_capability & old_s.cmd_input_capability) == 0))
    && (result == SCMI_SUCCESS ==> (qos_value == old_s.cmd_input_qos_value))
    && (result == SCMI_SUCCESS ==> (new_s.cmd_input_qos_value == old_s.cmd_input_qos_value))
    && (result == SCMI_SUCCESS ==> (new_s.cmd_input_domain_id == old_s.cmd_input_domain_id))
    && (result == SCMI_SUCCESS ==> (new_s.cmd_input_capability == old_s.cmd_input_capability))
}