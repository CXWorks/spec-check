pub open spec fn clock_rate_change_requested_notify__3_6_2_13_spec(result: int32, notify_enable: u32, old_s: S, new_s: S) -> bool {
    (!IsValidClockId(clock_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (ResultEqual(result, SUCCESS) && RateChangeRequestedNotifyEnabled(calling_agent(old_s), clock_id(old_s)) == notify_enable))
}