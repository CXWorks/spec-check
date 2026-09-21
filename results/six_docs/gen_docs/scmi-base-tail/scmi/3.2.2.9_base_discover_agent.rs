pub open spec fn 3.2.2.9_base_discover_agent_spec(result: int32, agent_id: UInt32, new_agent_id: UInt32, new_name: [UInt8; 16], old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (agent_id != 0xFFFFFFFF && new_agent_id == agent_id && new_name[0] != 0))
    && (result == SCMI_NOT_FOUND ==> (agent_id != 0 && agent_id != 0xFFFFFFFF && new_agent_id == agent_id && new_name[0] == 0))
    && (agent_id == 0xFFFFFFFF ==> (new_agent_id == agent_id && new_name[0] != 0))
    && (agent_id == 0 ==> (new_name[0] != 0 && new_name[0..5] == b"platform"))
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND ==> true)
}