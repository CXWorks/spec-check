pub open spec fn base_reset_agent_configuration__3_2_2_13_spec(result: i32, caller_agent_id: u32, agent_id: u32, flags: u32, old_s: S, new_s: S) -> bool {
    let not_supported = !BaseResetAgentConfigurationSupported(old_s);
    let not_found = !AgentExists(old_s, agent_id);
    let invalid_flags = (flags >> 1u32) != 0u32;
    let denied = !(caller_agent_id == agent_id || AgentMayResetAgentConfiguration(old_s, caller_agent_id, agent_id));
    let any_fail = not_supported || not_found || invalid_flags || denied;
    (not_supported ==> result == NOT_SUPPORTED)
    && (any_fail ==> (
        (not_supported && result == NOT_SUPPORTED)
        || (not_found && result == NOT_FOUND)
        || (invalid_flags && result == INVALID_PARAMETERS)
        || (denied && result == DENIED)
    ))
    && (any_fail ==> new_s == old_s)
    && (!any_fail ==> (
        result == SUCCESS
        && AgentPlatformResourceSettingsReset(old_s, new_s, agent_id)
        && SharedPlatformResourcesMeetRemainingAgentsRequirements(old_s, new_s, agent_id)
        && (((flags & 1u32) == 1u32) ==> AgentAccessPermissionsResetToDefault(old_s, new_s, agent_id))
        && (((flags & 1u32) == 0u32) ==> AgentAccessPermissionsUnchanged(old_s, new_s, agent_id))
    ))
}
