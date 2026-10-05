pub open spec fn base_set_protocol_permissions__3_2_2_12_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsCommandImplemented(BASE_SET_PROTOCOL_PERMISSIONS) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentExists(old_s, agent_id) ==> ResultEqual(result, NOT_FOUND))
    && (!DeviceExists(old_s, device_id) ==> ResultEqual(result, NOT_FOUND))
    && (!ProtocolExists(old_s, command_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidFlags(old_s, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!CallerMaySetProtocolPermissions(old_s, caller, agent_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> ProtocolPermission(new_s, agent_id, device_id, command_id) == (flags[0] == 1 ? ALLOWED : DENIED))
    && (ResultEqual(result, SUCCESS) ==> DeviceAccessPermission(new_s, agent_id, device_id) == DeviceAccessPermission(old_s, agent_id, device_id))
}