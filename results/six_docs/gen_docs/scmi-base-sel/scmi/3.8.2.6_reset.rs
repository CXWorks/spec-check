pub open spec fn 3.8.2.6_reset_spec(result: int, old_s: S, new_s: S) -> bool {
    ((result == SCMI_INVALID_PARAMETERS) ==> (old_s.flags & 0x7 != 0 || (old_s.flags & 0x4) != 0 || (old_s.flags & 0x2) != 0))
    && ((result == SCMI_NOT_FOUND) ==> (old_s.domain_id not in old_s.reset_domains))
    && ((result == SCMI_DENIED) ==> (old_s.agent not in old_s.reset_domain_permissions[old_s.domain_id]))
    && ((result == SCMI_GENERIC_ERROR) ==> (old_s.reset_domain_active[old_s.domain_id]))
    && (result == SCMI_SUCCESS ==> (new_s.reset_domain_state[old_s.domain_id] == old_s.reset_domain_state[old_s.domain_id]))
}