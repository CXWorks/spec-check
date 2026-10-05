pub open spec fn base_discover_agent__3_2_2_9_spec(agent_id: UInt32, status: int32, agent_id: UInt32, name: [uint8; 16], old_s: S, new_s: S) -> bool {
  (status == NOT_FOUND ==> agent_id == agent_id)
  && (status == SUCCESS && agent_id == 0xFFFFFFFF ==> agent_id == agent_id)
  && (status == SUCCESS && agent_id != 0xFFFFFFFF ==> agent_id == agent_id)
  && ((!(status == NOT_FOUND) &&
       status != SUCCESS)
    ==> agent_id == agent_id)
  && (status == SUCCESS && agent_id == 0 ==> name[0] == 'p')
  && (status != SUCCESS
    ==> name[0] == 0)
}