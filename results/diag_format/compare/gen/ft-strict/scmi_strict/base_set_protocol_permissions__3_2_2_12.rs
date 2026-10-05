pub open spec fn base_set_protocol_permissions__3_2_2_12_spec(agent_id: UInt32, device_id: UInt32, command_id: UInt32, flags: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!IsCommandSupported(old_s, BASE_SET_PROTOCOL_PERMISSIONS) ==> result == NOT_SUPPORTED)
  && (!AgentExists(old_s, agent_id) ==> result == NOT_FOUND)
  && (!DeviceExists(old_s, device_id) ==> result == NOT_FOUND)
  && (!ProtocolExists(old_s, Bits(command_id, 7, 0)) ==> result == NOT_FOUND)
  && (!IsValidFlags(old_s, flags) ==> result == INVALID_PARAMETERS)
  && (!CallerMaySetProtocolPermissions(old_s, caller, agent_id) ==> result == DENIED)
  && (result == SUCCESS ==> ProtocolAccessAllowed(new_s, agent_id, device_id, Bits(command_id, 7, 0)) == (Bits(flags, 0, 0) == 1))
  && (result == SUCCESS ==> DeviceAccessAllowed(new_s, agent_id, device_id) == DeviceAccessAllowed(old_s, agent_id, device_id))
  && ((IsCommandSupported(old_s, BASE_SET_PROTOCOL_PERMISSIONS) &&
       AgentExists(old_s, agent_id) &&
       DeviceExists(old_s, device_id) &&
       ProtocolExists(old_s, Bits(command_id, 7, 0)) &&
       IsValidFlags(old_s, flags) &&
       CallerMaySetProtocolPermissions(old_s, caller, agent_id))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> ProtocolAccessAllowed(new_s, agent_id, device_id, Bits(command_id, 7, 0)) == ProtocolAccessAllowed(old_s, agent_id, device_id, Bits(command_id, 7, 0)))
  && (result != RSI_SUCCESS
    ==> DeviceAccessAllowed(new_s, agent_id, device_id) == DeviceAccessAllowed(old_s, agent_id, device_id))
}