pub open spec fn performance_notify_limits__3_5_6_16_spec(
    result: Int32,
    old_s: S,
    new_s: S,
    domain_id: UInt32,
    notify_enable: UInt32,
) -> bool {
    (!IsValidPerfDomain(domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!PerfDomainSupportsLimitsNotify(domain_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidNotifyEnable(notify_enable) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> PerfLimitsNotifyEnabled(agent, domain_id) == (notify_enable[0] == 1))
}