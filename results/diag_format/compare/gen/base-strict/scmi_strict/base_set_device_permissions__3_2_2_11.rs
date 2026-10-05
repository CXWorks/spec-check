pub open spec fn base_set_device_permissions_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsCommandImplemented(old_s, 0x9, 0x10) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentExists(old_s, agent_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!DeviceExists(old_s, device_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidDevicePermissionFlags(old_s, flags(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!CallerMaySetAgentPermissions(old_s, caller(old_s), agent_id(old_s)) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (Bits64(flags(new_s), 0, 0) == 1 ==> AgentHasDeviceAccess(new_s, agent_id(old_s), device_id(old_s))))
    && (ResultEqual(result, SUCCESS) ==> (Bits64(flags(new_s), 0, 0) == 0 ==> !AgentHasDeviceAccess(new_s, agent_id(old_s), device_id(old_s))))
}