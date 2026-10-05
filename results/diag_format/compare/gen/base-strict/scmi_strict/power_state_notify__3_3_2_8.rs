pub open spec fn power_state_notify__3_3_2_8_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidPowerDomain(domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidNotifyEnable(notify_enable(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> (Bits(notify_enable(old_s), 0, 0) == 1 ==> PowerStateChangedNotifyEnabled(CallingAgent(), domain_id(old_s)) && Bits(notify_enable(old_s), 0, 0) == 0 ==> !PowerStateChangedNotifyEnabled(CallingAgent(), domain_id(old_s))))
}