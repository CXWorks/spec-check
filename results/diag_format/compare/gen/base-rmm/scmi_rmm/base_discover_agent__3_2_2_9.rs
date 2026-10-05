pub open spec fn base_discover_agent__3_2_2_9_spec(status: Int32, agent_id: UInt32, name: UInt8[16], old_s: S, new_s: S) -> bool {
    ((agent_id != 0xFFFFFFFF) && !IsValidAgentId(agent_id) ==> ResultEqual(status, NOT_FOUND))
    && (ResultEqual(status, SUCCESS) ==> (
        (agent_id == 0xFFFFFFFF ==> agent_id == CallingAgentId())
        && (agent_id != 0xFFFFFFFF ==> agent_id == agent_id)
        && IsNullTerminatedAscii(name)
        && StringLength(name) <= 16
        && (agent_id == 0 ==> StringStartsWith(name, "platform"))
        && name == AgentName(agent_id)
    ))
}