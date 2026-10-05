pub open spec fn base_reset_agent_configuration__3_2_2_13_spec(agent_id: UInt32, flags: UInt32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AgentExists(old_s, agent_id) ==> ResultEqual(result, NOT_FOUND))
  && (!AreValidResetFlags(old_s, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsCommandImplemented(old_s, BASE_RESET_AGENT_CONFIGURATION) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!CallerMayResetAgent(old_s, caller, agent_id) ==> ResultEqual(result, DENIED))
  && (result.is_Ok() ==> ResultEqual(result, SUCCESS))
  && (result.is_Ok() ==> ResourcesDedicatedTo(new_s, agent_id) are in their default state)
  && (result.is_Ok() ==> SharedResourcesUsedBy(new_s, agent_id) are in a state that still meets the requirements of the remaining agents that use them)
  && (result.is_Ok() && flags[0] == 1 ==> AccessPermissions(new_s, agent_id) == ImplDefinedDefaultPermissions(new_s, agent_id))
  && (result.is_Ok() && flags[0] == 0 ==> AccessPermissions(new_s, agent_id) is unchanged)
  && ((AgentExists(old_s, agent_id) &&
       AreValidResetFlags(old_s, flags) &&
       IsCommandImplemented(old_s, BASE_RESET_AGENT_CONFIGURATION) &&
       CallerMayResetAgent(old_s, caller, agent_id))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> AccessPermissions(new_s, agent_id) == AccessPermissions(old_s, agent_id))
}