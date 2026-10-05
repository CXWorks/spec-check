pub open spec fn base_set_device_permissions__3_2_2_11_spec(agent_id: UInt32, device_id: UInt32, flags: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsCommandImplemented(old_s, 0x10, 0x9) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentExists(old_s, agent_id) ==> ResultEqual(status, NOT_FOUND))
  && (!DeviceExists(old_s, device_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidDevicePermissionFlags(old_s, flags) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!CallerMaySetAgentPermissions(old_s, caller, agent_id) ==> ResultEqual(status, DENIED))
  && (result == SUCCESS ==> Bits(flags, 0, 0) == 1 ==> AgentHasDeviceAccess(new_s, agent_id, device_id))
  && (result == SUCCESS ==> Bits(flags, 0, 0) == 0 ==> !AgentHasDeviceAccess(new_s, agent_id, device_id))
  && ((!(IsCommandImplemented(old_s, 0x10, 0x9)) &&
       AgentExists(old_s, agent_id) &&
       DeviceExists(old_s, device_id) &&
       IsValidDevicePermissionFlags(old_s, flags) &&
       CallerMaySetAgentPermissions(old_s, caller, agent_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> !AgentHasDeviceAccess(new_s, agent_id, device_id))
}