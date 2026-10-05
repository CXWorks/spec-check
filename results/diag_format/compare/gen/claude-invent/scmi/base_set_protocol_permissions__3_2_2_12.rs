pub open spec fn base_set_protocol_permissions__3_2_2_12_spec(agent_id: u32, device_id: u32, command_id: u32, flags: u32, status: ScmiStatusCode, old_s: S, new_s: S) -> bool {
    (!BaseSetProtocolPermissionsSupported(old_s) ==> status == NOT_SUPPORTED)
    && ((!BaseSetProtocolPermissionsSupported(old_s)
        || !AgentExists(old_s, agent_id)
        || !DeviceExists(old_s, device_id)
        || !ProtocolExists(old_s, (command_id & 0xFFu32))
        || (flags & 0xFFFF_FFFEu32) != 0
        || !CallerMaySetProtocolPermissions(old_s, agent_id)) ==> (status != SUCCESS && new_s == old_s))
    && (status == NOT_FOUND ==> (!AgentExists(old_s, agent_id)
        || !DeviceExists(old_s, device_id)
        || !ProtocolExists(old_s, (command_id & 0xFFu32))))
    && (status == INVALID_PARAMETERS ==> (flags & 0xFFFF_FFFEu32) != 0)
    && (status == NOT_SUPPORTED ==> !BaseSetProtocolPermissionsSupported(old_s))
    && (status == DENIED ==> !CallerMaySetProtocolPermissions(old_s, agent_id))
    && ((BaseSetProtocolPermissionsSupported(old_s)
        && AgentExists(old_s, agent_id)
        && DeviceExists(old_s, device_id)
        && ProtocolExists(old_s, (command_id & 0xFFu32))
        && (flags & 0xFFFF_FFFEu32) == 0
        && CallerMaySetProtocolPermissions(old_s, agent_id)) ==> (
            status == SUCCESS
            && AgentProtocolAccessAllowed(new_s, agent_id, device_id, (command_id & 0xFFu32)) == ((flags & 1u32) == 1u32)
            && (forall|a: u32, d: u32, p: u32|
                !(a == agent_id && d == device_id && p == (command_id & 0xFFu32)) ==>
                    AgentProtocolAccessAllowed(new_s, a, d, p) == AgentProtocolAccessAllowed(old_s, a, d, p))
            && (forall|a: u32, d: u32|
                AgentDeviceAccessAllowed(new_s, a, d) == AgentDeviceAccessAllowed(old_s, a, d))
        ))
}
