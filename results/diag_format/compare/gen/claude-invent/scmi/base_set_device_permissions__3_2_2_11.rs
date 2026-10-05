pub open spec fn base_set_device_permissions__3_2_2_11_spec(agent_id: u32, device_id: u32, flags: u32, result: i32, old_s: S, new_s: S) -> bool {
    (!BaseSetDevicePermissionsSupported(old_s) ==> result == NOT_SUPPORTED)
    && ((!AgentExists(old_s, agent_id) || !DeviceExists(old_s, device_id)) ==> result != SUCCESS)
    && ((flags & 0xFFFF_FFFEu32) != 0 ==> result != SUCCESS)
    && (!CallerAllowedToSetAgentPermissions(old_s, agent_id) ==> result != SUCCESS)
    && (result == NOT_FOUND ==> (!AgentExists(old_s, agent_id) || !DeviceExists(old_s, device_id)))
    && (result == INVALID_PARAMETERS ==> (flags & 0xFFFF_FFFEu32) != 0)
    && (result == DENIED ==> !CallerAllowedToSetAgentPermissions(old_s, agent_id))
    && (result == NOT_SUPPORTED ==> !BaseSetDevicePermissionsSupported(old_s))
    && (result != SUCCESS ==> new_s == old_s)
    && ((BaseSetDevicePermissionsSupported(old_s)
        && AgentExists(old_s, agent_id)
        && DeviceExists(old_s, device_id)
        && (flags & 0xFFFF_FFFEu32) == 0
        && CallerAllowedToSetAgentPermissions(old_s, agent_id))
        ==> (result == SUCCESS
            && AgentDeviceAccessAllowed(new_s, agent_id, device_id) == ((flags & 1u32) == 1u32)
            && DevicePermissionsUnchangedExcept(old_s, new_s, agent_id, device_id)))
}
