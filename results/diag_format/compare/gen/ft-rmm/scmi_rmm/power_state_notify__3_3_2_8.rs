pub open spec fn power_state_notify__3_3_2_8_spec(domain_id: UInt32, notify_enable: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidPowerDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
  && (!IsValidNotifyEnable(old_s, notify_enable) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> PowerStateNotifyEnabled(new_s, calling_agent, domain_id) == (notify_enable[0] == 1))
  && ((IsValidPowerDomain(old_s, domain_id) &&
       IsValidNotifyEnable(old_s, notify_enable))
    ==> ResultEqual(result, SUCCESS))
  && (result != SUCCESS
    ==> PowerStateNotifyEnabled(new_s, calling_agent, domain_id) == PowerStateNotifyEnabled(old_s, calling_agent, domain_id))
}