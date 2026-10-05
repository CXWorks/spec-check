pub open spec fn clock_rate_change_requested_notify__3_6_2_13_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!IsValidClockId(old_s, clock_id) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (Bits64(notify_enable, 0, 0) == 1 ==> RateChangeRequestedNotifyEnabled(new_s, CallingAgent(), clock_id))
    && (Bits64(notify_enable, 0, 0) == 0 ==> !RateChangeRequestedNotifyEnabled(new_s, CallingAgent(), clock_id)))
}