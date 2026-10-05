pub open spec fn base_notify_errors__3_2_2_10_spec(notify_enable: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidNotifyEnable(old_s, notify_enable) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) && (Bits(notify_enable, 0, 0) == 1) ==> ErrorEventNotificationsEnabled(new_s, CallingAgent()))
  && (ResultEqual(status, SUCCESS) && (Bits(notify_enable, 0, 0) == 0) ==> !ErrorEventNotificationsEnabled(new_s, CallingAgent()))
  && ((IsValidNotifyEnable(old_s, notify_enable))
    ==> ResultEqual(status, SUCCESS))
}