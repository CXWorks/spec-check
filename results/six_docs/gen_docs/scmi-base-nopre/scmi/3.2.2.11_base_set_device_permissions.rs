pub open spec fn base_set_device_permissions_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (new_s.agent_permissions == old_s.agent_permissions))
    && (result == 0 ==> (new_s.device_permissions == old_s.device_permissions))
    && (result == 0 ==> (new_s.agent_id == old_s.agent_id))
    && (result == 0 ==> (new_s.device_id == old_s.device_id))
    && (result == 0 ==> (new_s.flags == old_s.flags))
    && (result != 0 ==> (result == -1 || result == -2 || result == -3 || result == -4 || result == -5))
}