pub open spec fn power_state_notify__3_3_2_8_spec(domain_id: UInt32, notify_enable: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidPowerDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidNotifyEnable(old_s, notify_enable) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) && Bits(notify_enable, 0, 0) == 1 ==> PowerStateChangedNotifyEnabled(new_s, CallingAgent(), domain_id))
  && (ResultEqual(status, SUCCESS) && Bits(notify_enable, 0, 0) == 0 ==> !PowerStateChangedNotifyEnabled(new_s, CallingAgent(), domain_id))
  && ((IsValidPowerDomain(old_s, domain_id) &&
       IsValidNotifyEnable(old_s, notify_enable))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> !PowerStateChangedNotifyEnabled(new_s, CallingAgent(), domain_id))
}