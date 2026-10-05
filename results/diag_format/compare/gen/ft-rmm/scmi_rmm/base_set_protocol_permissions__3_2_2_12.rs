pub open spec fn base_set_protocol_permissions__3_2_2_12_spec(agent_id: UInt32, device_id: UInt32, command_id: UInt32, flags: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsCommandImplemented(old_s, BASE_SET_PROTOCOL_PERMISSIONS) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentExists(old_s, agent_id) ==> ResultEqual(status, NOT_FOUND))
  && (!DeviceExists(old_s, device_id) ==> ResultEqual(status, NOT_FOUND))
  && (!ProtocolExists(old_s, command_id as int) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidFlags(old_s, flags) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!CallerMaySetProtocolPermissions(old_s, caller, agent_id) ==> ResultEqual(status, DENIED))
  && (result == SUCCESS ==> ProtocolPermission(new_s, agent_id, device_id, command_id as int) == (flags[0] == 1 ? ALLOWED : DENIED))
  && (result == SUCCESS ==> DeviceAccessPermission(new_s, agent_id, device_id) == DeviceAccessPermission(old_s, agent_id, device_id))
  && ((IsCommandImplemented(old_s, BASE_SET_PROTOCOL_PERMISSIONS) &&
       AgentExists(old_s, agent_id) &&
       DeviceExists(old_s, device_id) &&
       ProtocolExists(old_s, command_id as int) &&
       IsValidFlags(old_s, flags) &&
       CallerMaySetProtocolPermissions(old_s, caller, agent_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> ProtocolPermission(new_s, agent_id, device_id, command_id as int) == ProtocolPermission(old_s, agent_id, device_id, command_id as int))
  && (result != SUCCESS
    ==> DeviceAccessPermission(new_s, agent_id, device_id) == DeviceAccessPermission(old_s, agent_id, device_id))
}