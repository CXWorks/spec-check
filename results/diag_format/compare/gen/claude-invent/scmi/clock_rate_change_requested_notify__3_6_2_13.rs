pub open spec fn clock_rate_change_requested_notify__3_6_2_13_spec(clock_id: UInt32, notify_enable: UInt32, status: i32, old_s: S, new_s: S) -> bool {
    (!ClockIdIsValid(old_s, clock_id) ==> (status == NOT_FOUND && new_s == old_s))
    && (status == SUCCESS ==> (
        ClockIdIsValid(old_s, clock_id)
        && ClockRateChangeRequestedNotifyEnabled(new_s, clock_id) == ((notify_enable & 1u32) == 1u32)
    ))
}
