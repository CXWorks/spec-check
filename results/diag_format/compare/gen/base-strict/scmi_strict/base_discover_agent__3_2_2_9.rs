pub open spec fn base_discover_agent__3_2_2_9_spec(result: Int32, out_agent_id: UInt32, name: UInt8[16], old_s: S, new_s: S) -> bool {
    (agent_id != 0xFFFFFFFF && !IsValidAgentId(agent_id) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (agent_id == 0xFFFFFFFF ==> out_agent_id == CallingAgentId()))
    && (ResultEqual(result, SUCCESS) ==> (agent_id != 0xFFFFFFFF ==> out_agent_id == agent_id))
    && (ResultEqual(result, SUCCESS) ==> name == AgentName(out_agent_id))
    && (ResultEqual(result, SUCCESS) ==> IsNullTerminatedAscii(name, 16))
    && (ResultEqual(result, SUCCESS) ==> (out_agent_id == 0 ==> NameStartsWithPlatform(name)))
    && (old_s == new_s)
}