pub open spec fn base_reset_agent_configuration__3_2_2_13_spec(status: Int32, old_s: S, new_s: S) -> bool {
    (!AgentExists(old_s, agent_id) ==> ResultEqual(status, NOT_FOUND))
    && (!AreValidResetFlags(old_s, flags) ==> ResultEqual(status, INVALID_PARAMETERS))
    && (!IsCommandImplemented(old_s, BASE_RESET_AGENT_CONFIGURATION) ==> ResultEqual(status, NOT_SUPPORTED))
    && (!CallerMayResetAgent(old_s, caller, agent_id) ==> ResultEqual(status, DENIED))
    && (ResultEqual(status, SUCCESS) ==> ResourcesDedicatedTo(old_s, agent_id) == ResourcesDedicatedTo(new_s, agent_id))
    && (ResultEqual(status, SUCCESS) ==> SharedResourcesUsedBy(old_s, agent_id) == SharedResourcesUsedBy(new_s, agent_id))
    && (flags[0] == 1 ==> AccessPermissions(old_s, agent_id) == ImplDefinedDefaultPermissions(old_s, agent_id))
    && (flags[0] == 0 ==> AccessPermissions(old_s, agent_id) == AccessPermissions(new_s, agent_id))
}