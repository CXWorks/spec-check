pub open spec fn base_set_protocol_permissions__3_2_2_12_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0x80000000 ==> (old_s.agent_exists(old_s.agent_id) == false || old_s.device_exists(old_s.device_id) == false || old_s.protocol_exists(old_s.protocol_id) == false))
    && (result == 0x80000001 ==> (old_s.flags_valid(old_s.flags) == false))
    && (result == 0x80000002 ==> (old_s.command_supported(old_s.command_id) == false))
    && (result == 0x80000003 ==> (old_s.agent_allowed_to_set_permissions(old_s.agent_id, old_s.agent_id) == false))
    && (result == 0 ==> (old_s.agent_exists(old_s.agent_id) && old_s.device_exists(old_s.device_id) && old_s.protocol_exists(old_s.protocol_id) && old_s.flags_valid(old_s.flags) && old_s.command_supported(old_s.command_id) && old_s.agent_allowed_to_set_permissions(old_s.agent_id, old_s.agent_id) && new_s.agent_protocol_permissions(old_s.agent_id, old_s.device_id, old_s.protocol_id) == (old_s.agent_protocol_permissions(old_s.agent_id, old_s.device_id, old_s.protocol_id) | (old_s.flags & 0x1) << 0)))
}