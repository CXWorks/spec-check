pub open spec fn base_reset_agent_configuration__3_2_2_13_spec(agent_id: UInt32, flags: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsCommandSupported(old_s, BASE_RESET_AGENT_CONFIGURATION) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentExists(old_s, agent_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidResetAgentFlags(old_s, flags) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!CallerMayResetAgentConfiguration(old_s, caller_agent_id, agent_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> DedicatedResourcesInDefaultState(new_s, agent_id))
  && (ResultEqual(status, SUCCESS) ==> SharedResourcesMeetRemainingAgentsRequirements(new_s, agent_id))
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 0, 0) == 1 ==> AgentPermissionsAreImplementationDefinedDefaults(new_s, agent_id))
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 0, 0) == 0 ==> AgentPermissionsUnchanged(new_s, agent_id))
  && ((!(IsCommandSupported(old_s, BASE_RESET_AGENT_CONFIGURATION)) &&
       AgentExists(old_s, agent_id) &&
       IsValidResetAgentFlags(old_s, flags) &&
       CallerMayResetAgentConfiguration(old_s, caller_agent_id, agent_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> DedicatedResourcesInDefaultState(new_s, agent_id))
  && (result != SUCCESS
    ==> SharedResourcesMeetRemainingAgentsRequirements(new_s, agent_id))
  && (result != SUCCESS
    ==> AgentPermissionsAreImplementationDefinedDefaults(new_s, agent_id))
  && (result != SUCCESS
    ==> AgentPermissionsUnchanged(new_s, agent_id))
}