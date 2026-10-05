pub open spec fn base_discover_agent__3_2_2_9_spec(agent_id: UInt32, status: Int32, out_agent_id: UInt32, name: [UInt8; 16], result: Result, old_s: S, new_s: S) -> bool {
  (agent_id != 0xFFFFFFFF && !IsValidAgentId(agent_id) ==> ResultEqual(status, NOT_FOUND))
  && (result == SUCCESS ==> agent_id == 0xFFFFFFFF ==> out_agent_id == CallingAgentId())
  && (result == SUCCESS ==> agent_id != 0xFFFFFFFF ==> out_agent_id == agent_id)
  && (result == SUCCESS ==> name == AgentName(new_s, out_agent_id))
  && (result == SUCCESS ==> IsNullTerminatedAscii(name, 16))
  && (result == SUCCESS ==> out_agent_id == 0 ==> NameStartsWithPlatform(name))
  && ((!(agent_id != 0xFFFFFFFF && !IsValidAgentId(agent_id)))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> out_agent_id == 0xFFFFFFFF)
  && (result != SUCCESS
    ==> out_agent_id == agent_id)
  && (result != SUCCESS
    ==> name == AgentName(new_s, out_agent_id))
  && (result != SUCCESS
    ==> IsNullTerminatedAscii(name, 16))
  && (result != SUCCESS
    ==> NameStartsWithPlatform(name))
}