pub open spec fn power_state_change_requested_notify__3_3_2_9_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidPowerDomain(domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (IsValidPowerDomain(domain_id(old_s)) ==> ResultEqual(result, SUCCESS) && PowerStateChangeRequestedNotifyEnabled(caller(old_s), domain_id(old_s)) == notify_enable(old_s)[0])
}