pub open spec fn 3.2.2.12_base_set_protocol_permissions_spec(agent_id: UInt32, device_id: UInt32, command_id: UInt32, flags: UInt32, result: Result<(), int>, old_s: S, new_s: S) -> bool {
  (result == SCMI_SUCCESS)
}