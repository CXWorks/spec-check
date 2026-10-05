pub open spec fn system_power_state_notify__3_4_2_7_spec(status: Int32, notify_enable: UInt32, old_s: S, new_s: S) -> bool {
    (!NotificationsSupportedForAgent(old_s, caller) ==> ResultEqual(status, NOT_SUPPORTED))
    && (!IsValidNotifyEnable(notify_enable) ==> ResultEqual(status, INVALID_PARAMETERS))
    && (ResultEqual(status, SUCCESS) ==> SystemPowerStateNotifyEnabled(caller) == (notify_enable[0] == 1))
    && (ResultEqual(status, SUCCESS) ==> SystemPowerStateNotifyEnabled(caller) == old_s.SystemPowerStateNotifyEnabled(caller))
}