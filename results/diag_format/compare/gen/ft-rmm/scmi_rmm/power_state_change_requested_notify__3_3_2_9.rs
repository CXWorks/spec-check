pub open spec fn power_state_change_requested_notify__3_3_2_9_spec(domain_id: UInt32, notify_enable: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidPowerDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (result: Result<(), RmiStatusCode>; (result.is_Ok() ==> ResultEqual(status, SUCCESS))
    && (result.is_Ok() ==> PowerStateChangeRequestedNotifyEnabled(new_s, caller, domain_id) == notify_enable[0]))
  && ((IsValidPowerDomain(old_s, domain_id))
    ==> ResultEqual(status, SUCCESS))
}