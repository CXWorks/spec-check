pub open spec fn clock_rate_notify_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidClockDevice(clock_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidNotifyEnable(notify_enable(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> (Bits(notify_enable(old_s), 0, 0) == 1 ==> ClockRateNotifyEnabled(CallingAgent(), clock_id(old_s)))
    && (Bits(notify_enable(old_s), 0, 0) == 0 ==> !ClockRateNotifyEnabled(CallingAgent(), clock_id(old_s))))
}