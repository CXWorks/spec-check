pub open spec fn base_set_device_permissions__3_2_2_11_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!AgentExists(old_s, agent_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!DeviceExists(old_s, device_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidDevicePermissionFlags(old_s, flags(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsCommandSupported(old_s, BASE_SET_DEVICE_PERMISSIONS) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!CallerMaySetPermissionsOf(old_s, agent_id(old_s)) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (
        (flags(old_s).access_type == 1 ==> DeviceAccessAllowed(new_s, agent_id(old_s), device_id(old_s)))
        && (flags(old_s).access_type == 0 ==> !DeviceAccessAllowed(new_s, agent_id(old_s), device_id(old_s)))
    ))
}