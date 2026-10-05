pub open spec fn performance_notify_limits__3_5_6_16_spec(domain_id: UInt32, notify_enable: UInt32, status: i32, old_s: S, new_s: S) -> bool {
    (!IsValidPerfDomain(old_s, domain_id) ==> status == NOT_FOUND)
    && ((IsValidPerfDomain(old_s, domain_id)
         && !PerfDomainSupportsLimitsNotify(old_s, domain_id)) ==> status == NOT_SUPPORTED)
    && ((IsValidPerfDomain(old_s, domain_id)
         && PerfDomainSupportsLimitsNotify(old_s, domain_id)
         && (notify_enable & 0xFFFF_FFFEu32) != 0) ==> status == INVALID_PARAMETERS)
    && ((status != SUCCESS) ==> new_s == old_s)
    && ((IsValidPerfDomain(old_s, domain_id)
         && PerfDomainSupportsLimitsNotify(old_s, domain_id)
         && (notify_enable & 0xFFFF_FFFEu32) == 0) ==> (
            status == SUCCESS
            && PerfLimitsNotifyEnabled(new_s, CallingAgent(old_s), domain_id) == ((notify_enable & 1u32) == 1u32)
            && (forall|a: UInt32, d: UInt32|
                    !(a == CallingAgent(old_s) && d == domain_id)
                    ==> PerfLimitsNotifyEnabled(new_s, a, d) == PerfLimitsNotifyEnabled(old_s, a, d))
         ))
}
