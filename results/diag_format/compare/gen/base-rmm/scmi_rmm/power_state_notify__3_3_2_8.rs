pub open spec fn power_state_notify__3_3_2_8_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidPowerDomain(old_s, domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidNotifyEnable(old_s, notify_enable(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> PowerStateNotifyEnabled(new_s, calling_agent(old_s), domain_id(old_s)) == (notify_enable(old_s)[0] == 1))
}