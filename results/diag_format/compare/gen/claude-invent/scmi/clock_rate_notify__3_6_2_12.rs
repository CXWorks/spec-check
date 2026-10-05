pub open spec fn clock_rate_notify__3_6_2_12_spec(clock_id: UInt32, notify_enable: UInt32, status: i32, old_s: S, new_s: S) -> bool {
    (!IsValidClockDevice(old_s, clock_id) ==> status == NOT_FOUND)
    && ((IsValidClockDevice(old_s, clock_id) && (notify_enable & 0xFFFF_FFFEu32) != 0) ==> status == INVALID_PARAMETERS)
    && (status != SUCCESS ==> new_s == old_s)
    && (status == SUCCESS ==> (
        IsValidClockDevice(old_s, clock_id)
        && (notify_enable & 0xFFFF_FFFEu32) == 0
        && ClockRateNotifyEnabled(new_s, clock_id) == ((notify_enable & 1u32) == 1u32)
        && ClockStateUnchangedExceptNotify(old_s, new_s, clock_id)
    ))
}
