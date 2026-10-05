pub open spec fn system_power_state_notify__3_4_2_7_spec(notify_enable: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!NotificationsSupportedForAgent(old_s, caller) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!IsValidNotifyEnable(old_s, notify_enable) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> SystemPowerStateNotifyEnabled(new_s, caller) == (notify_enable[0] == 1))
  && ((NotificationsSupportedForAgent(old_s, caller) &&
       IsValidNotifyEnable(old_s, notify_enable))
    ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, NOT_SUPPORTED)
    ==> SystemPowerStateNotifyEnabled(new_s, caller) == SystemPowerStateNotifyEnabled(old_s, caller))
  && (ResultEqual(status, INVALID_PARAMETERS)
    ==> SystemPowerStateNotifyEnabled(new_s, caller) == SystemPowerStateNotifyEnabled(old_s, caller))
}