pub open spec fn base_set_device_permissions__3_2_2_11_spec(agent_id: UInt32, device_id: UInt32, flags: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!AgentExists(old_s, agent_id) ==> ResultEqual(status, NOT_FOUND))
  && (!DeviceExists(old_s, device_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidDevicePermissionFlags(old_s, flags) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsCommandSupported(old_s, BASE_SET_DEVICE_PERMISSIONS) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!CallerMaySetPermissionsOf(old_s, agent_id) ==> ResultEqual(status, DENIED))
  && (result == SUCCESS ==> (flags.access_type == 1) ==> DeviceAccessAllowed(new_s, agent_id, device_id))
  && (result == SUCCESS ==> (flags.access_type == 0) ==> !DeviceAccessAllowed(new_s, agent_id, device_id))
  && ((!(AgentExists(old_s, agent_id)) &&
       DeviceExists(old_s, device_id) &&
       IsValidDevicePermissionFlags(old_s, flags) &&
       IsCommandSupported(old_s, BASE_SET_DEVICE_PERMISSIONS) &&
       CallerMaySetPermissionsOf(old_s, agent_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> !DeviceAccessAllowed(new_s, agent_id, device_id))
  && (result != SUCCESS
    ==> DeviceAccessAllowed(new_s, agent_id, device_id))
}