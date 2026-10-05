pub open spec fn base_notify_errors__3_2_2_10_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!IsValidNotifyEnable(notify_enable) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ResultEqual(result, RMI_ERROR_INPUT) ==> !IsValidNotifyEnable(notify_enable))
    && (ResultEqual(result, RMI_SUCCESS) ==> (Bits(notify_enable, 0, 0) == 1 ==> ErrorEventNotificationsEnabled(CallingAgent()) && Bits(notify_enable, 0, 0) == 0 ==> !ErrorEventNotificationsEnabled(CallingAgent())))
}