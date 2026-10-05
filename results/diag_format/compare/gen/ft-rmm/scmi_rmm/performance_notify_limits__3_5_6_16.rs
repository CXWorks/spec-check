pub open spec fn performance_notify_limits__3_5_6_16_spec(domain_id: UInt32, notify_enable: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidPerfDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!PerfDomainSupportsLimitsNotify(old_s, domain_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!IsValidNotifyEnable(old_s, notify_enable) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> PerfLimitsNotifyEnabled(new_s, agent, domain_id) == (notify_enable[0] == 1))
  && ((IsValidPerfDomain(old_s, domain_id) &&
       PerfDomainSupportsLimitsNotify(old_s, domain_id) &&
       IsValidNotifyEnable(old_s, notify_enable))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> PerfLimitsNotifyEnabled(new_s, agent, domain_id) == PerfLimitsNotifyEnabled(old_s, agent, domain_id))
}