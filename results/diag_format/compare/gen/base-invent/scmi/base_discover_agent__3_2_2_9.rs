pub open spec fn base_discover_agent__3_2_2_9_spec(result: int32, agent_id: UInt32, new_agent_id: UInt32, new_name: [UInt8; 16], old_s: S, new_s: S) -> bool {
    (agent_id == 0xFFFFFFFF ==> (result == SUCCESS && new_agent_id == agent_id && new_name[0] == 0))
    && (agent_id == 0 ==> (result == SUCCESS && new_agent_id == agent_id && new_name[0] == 'p' && new_name[1] == 'l' && new_name[2] == 'a' && new_name[3] == 't' && new_name[4] == 'f' && new_name[5] == 'o' && new_name[6] == 'r' && new_name[7] == 'm' && new_name[8] == ' ')
    && (agent_id != 0 && agent_id != 0xFFFFFFFF ==> (result == SUCCESS || result == NOT_FOUND))
    && (result == NOT_FOUND ==> (new_agent_id == agent_id && new_name[0] == 0))
    && (result == SUCCESS ==> (new_agent_id == agent_id && new_name[0] != 0))
    && (result != SUCCESS && result != NOT_FOUND && result != 0 ==> true)
}