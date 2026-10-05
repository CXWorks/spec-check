pub open spec fn reset_notify__3_8_2_7_spec(domain_id: UInt32, notify_enable: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidResetDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidNotifyEnable(old_s, notify_enable) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) && Bits(notify_enable, 0, 0) == 1 ==> ResetNotificationsEnabled(new_s, CallingAgent(), domain_id))
  && (ResultEqual(status, SUCCESS) && Bits(notify_enable, 0, 0) == 0 ==> !ResetNotificationsEnabled(new_s, CallingAgent(), domain_id))
  && ((IsValidResetDomain(old_s, domain_id) &&
       IsValidNotifyEnable(old_s, notify_enable))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> !ResetNotificationsEnabled(new_s, CallingAgent(), domain_id))
}