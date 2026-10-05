pub open spec fn power_state_change_requested_notify__3_3_2_9_spec(domain_id: UInt32, notify_enable: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidPowerDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) && Bits(notify_enable, 0, 0) == 1 ==> PowerStateChangeRequestedNotifyEnabled(new_s, CallingAgent(), domain_id))
  && (ResultEqual(status, SUCCESS) && Bits(notify_enable, 0, 0) == 0 ==> !PowerStateChangeRequestedNotifyEnabled(new_s, CallingAgent(), domain_id))
  && ((IsValidPowerDomain(old_s, domain_id))
    ==> ResultEqual(status, SUCCESS))
}