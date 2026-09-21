pub open spec fn 3.2.2.12_base_set_protocol_permissions_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (new_s.agent_permissions(old_s.agent_id, old_s.device_id, old_s.protocol_id) == old_s.agent_permissions(old_s.agent_id, old_s.device_id, old_s.protocol_id)))
    && (result != 0 ==> (new_s.agent_permissions(old_s.agent_id, old_s.device_id, old_s.protocol_id) != old_s.agent_permissions(old_s.agent_id, old_s.device_id, old_s.protocol_id)))
}