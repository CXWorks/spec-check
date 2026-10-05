pub open spec fn power_state_change_requested_notify__3_3_2_9_spec(domain_id: u32, notify_enable: u32, agent_id: u32, result: i32, old_s: S, new_s: S) -> bool {
    (!IsValidPowerDomain(old_s, domain_id) ==> (result == NOT_FOUND && new_s == old_s))
    && (result == SUCCESS ==> (
        IsValidPowerDomain(old_s, domain_id)
        && PowerStateChangeRequestedNotifyEnabled(new_s, agent_id, domain_id) == ((notify_enable & 1u32) == 1u32)
    ))
    && (result != SUCCESS ==> new_s == old_s)
}
