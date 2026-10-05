pub open spec fn reset_notify__3_8_2_7_spec(result: int32, old_s: S, new_s: S) -> bool {
    (!IsValidResetDomain(domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidNotifyEnable(notify_enable(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> ResetNotifyEnabled(domain_id(old_s), calling_agent(old_s)) == (notify_enable(old_s)[0] == 1))
}