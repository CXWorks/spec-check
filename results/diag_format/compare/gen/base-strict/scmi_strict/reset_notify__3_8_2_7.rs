pub open spec fn reset_notify__3_8_2_7_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidResetDomain(domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidNotifyEnable(notify_enable(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> (Bits(notify_enable(old_s), 0, 0) == 1 ==> ResetNotificationsEnabled(CallingAgent(), domain_id(old_s))) && (Bits(notify_enable(old_s), 0, 0) == 0 ==> !ResetNotificationsEnabled(CallingAgent(), domain_id(old_s))))
}