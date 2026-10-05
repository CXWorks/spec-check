pub open spec fn base_set_protocol_permissions__3_2_2_12_spec(status: Int32, old_s: S, new_s: S) -> bool {
    (!IsCommandSupported(BASE_SET_PROTOCOL_PERMISSIONS) ==> ResultEqual(status, NOT_SUPPORTED))
    && (!AgentExists(old_s, agent_id) ==> ResultEqual(status, NOT_FOUND))
    && (!DeviceExists(old_s, device_id) ==> ResultEqual(status, NOT_FOUND))
    && (!ProtocolExists(old_s, Bits(command_id, 7, 0)) ==> ResultEqual(status, NOT_FOUND))
    && (!IsValidFlags(old_s, flags) ==> ResultEqual(status, INVALID_PARAMETERS))
    && (!CallerMaySetProtocolPermissions(old_s, caller, agent_id) ==> ResultEqual(status, DENIED))
    && (ResultEqual(status, SUCCESS) ==> ProtocolAccessAllowed(new_s, agent_id, device_id, Bits(command_id, 7, 0)) == (Bits(flags, 0, 0) == 1))
    && (ResultEqual(status, SUCCESS) ==> DeviceAccessAllowed(new_s, agent_id, device_id) == DeviceAccessAllowed(old_s, agent_id, device_id))
}