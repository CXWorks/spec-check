pub open spec fn 3.2.2.12_base_set_protocol_permissions_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (new_s.agent_permissions == old_s.agent_permissions))
    && (result == SCMI_SUCCESS ==> (new_s.agent_permissions[old_s.agent_id][old_s.device_id][old_s.protocol_id] == old_s.flags))
    && (result == SCMI_NOT_FOUND ==> (old_s.agent_id >= old_s.agent_count || old_s.device_id >= old_s.device_count || old_s.protocol_id >= old_s.protocol_count))
    && (result == SCMI_INVALID_PARAMETERS ==> (old_s.flags != 0))
    && (result == SCMI_NOT_SUPPORTED ==> true)
    && (result == SCMI_DENIED ==> true)
}