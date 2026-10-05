pub open spec fn base_reset_agent_configuration__3_2_2_13_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> !AgentExists(old_s, agent_id))
    && (result == INVALID_PARAMETERS ==> flags != 0)
    && (result == NOT_SUPPORTED ==> !CommandSupported(old_s, 0xB))
    && (result == DENIED ==> !AgentCanReset(old_s, caller_id, agent_id))
    && (result == SUCCESS ==> AgentExists(old_s, agent_id) && flags == 0 && CommandSupported(old_s, 0xB) && AgentCanReset(old_s, caller_id, agent_id))
    && (result == SUCCESS ==> AgentPermissionsReset(old_s, agent_id, new_s))
    && (result == SUCCESS ==> PlatformResourcesReset(old_s, agent_id, new_s))
}