pub open spec fn clock_rate_notify_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!IsValidClockDevice(old_s, clock_id) ==> ResultEqual(result, RMI_ERROR_NOT_FOUND))
    && (!IsValidNotifyEnable(old_s, notify_enable) ==> ResultEqual(result, RMI_ERROR_INVALID_PARAMETERS))
    && (ResultEqual(result, RMI_SUCCESS) ==> ClockRateNotifyEnabled(new_s, calling_agent, clock_id) == (notify_enable[0] == 1))
}