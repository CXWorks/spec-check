pub open spec fn performance_notify_limits__3_5_6_16_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidPerfDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!PerfDomainSupportsLimitsNotify(old_s, domain_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidNotifyEnable(old_s, notify_enable) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> (
        (Bits(notify_enable, 0, 0) == 1 ==> LimitsNotifyEnabled(CallingAgent(), domain_id))
        && (Bits(notify_enable, 0, 0) == 0 ==> !LimitsNotifyEnabled(CallingAgent(), domain_id))
    ))
}