pub open spec fn base_set_device_permissions__3_2_2_11_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (old_s.agent_permissions.contains(old_s.agent_id, old_s.device_id, 1) && new_s.agent_permissions.contains(new_s.agent_id, new_s.device_id, 1)))
    && (result != 0 ==> (result == -1 || result == -2 || result == -3 || result == -4 || result == -5))
}