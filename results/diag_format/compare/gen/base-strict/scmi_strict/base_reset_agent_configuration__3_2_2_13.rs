pub open spec fn base_reset_agent_configuration__3_2_2_13_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsCommandSupported(BASE_RESET_AGENT_CONFIGURATION) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentExists(old_s, agent_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidResetAgentFlags(old_s, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!CallerMayResetAgentConfiguration(old_s, caller_agent_id, agent_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> DedicatedResourcesInDefaultState(old_s, agent_id))
    && (ResultEqual(result, SUCCESS) ==> SharedResourcesMeetRemainingAgentsRequirements(old_s, agent_id))
    && (ResultEqual(result, SUCCESS) ==> (Bits(flags, 0, 0) == 1 ==> AgentPermissionsAreImplementationDefinedDefaults(old_s, agent_id)))
    && (ResultEqual(result, SUCCESS) ==> (Bits(flags, 0, 0) == 0 ==> AgentPermissionsUnchanged(old_s, agent_id)))
}