pub open spec fn performance_domain_attributes__3_5_6_5_spec(result: Int32, attributes: UInt32, rate_limit: UInt32, sustained_freq: UInt32, sustained_perf_level: UInt32, name: UInt8[16], guaranteed_perf_level: UInt32, qos_capability_types: UInt32, qos_parent_id: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidPerformanceDomain(domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (
        (Bits(attributes, 31, 31) == 1) == CallerCanSetPerfLimits(domain_id(old_s))
        && (Bits(attributes, 30, 30) == 1) == CallerCanSetPerfLevel(domain_id(old_s))
        && (Bits(attributes, 29, 29) == 1) == SupportsPerfLimitsChangeNotify(domain_id(old_s))
        && (Bits(attributes, 28, 28) == 1) == SupportsPerfLevelChangeNotify(domain_id(old_s))
        && (Bits(attributes, 27, 27) == 1) == HasFastChannel(domain_id(old_s))
        && (Bits(attributes, 26, 26) == 1) == (PerfDomainNameLength(domain_id(old_s)) > 16)
        && (Bits(attributes, 25, 25) == 1) == UsesLevelIndexingMode(domain_id(old_s))
        && (Bits(attributes, 24, 24) == 1) == SupportsAsyncQosConfig(domain_id(old_s))
        && (Bits(attributes, 23, 23) == 1) == IsQosOnlyDomain(domain_id(old_s))
        && (Bits(attributes, 23, 23) == 1 ==> Bits(attributes, 31, 28) == 0)
        && (Bits(attributes, 22, 22) == 1) == SupportsSustainedPerfReduction(domain_id(old_s))
        && (Bits(attributes, 23, 23) == 1 ==> Bits(attributes, 22, 22) == 0)
        && Bits(attributes, 21, 0) == 0
        && Bits(rate_limit, 31, 20) == 0
        && Bits(rate_limit, 19, 0) == RateLimitMicroseconds(domain_id(old_s))
        && (sustained_freq == SustainedFreqKhz(domain_id(old_s)) || (Bits(attributes, 23, 23) == 1 && sustained_freq == 0))
        && (sustained_perf_level == PlatformSustainedPerfLevel(domain_id(old_s)) || (Bits(attributes, 23, 23) == 1 && sustained_perf_level == 0))
        && IsNullTerminatedAscii(name, 16)
        && (Bits(attributes, 26, 26) == 0 ==> name == PerfDomainName(domain_id(old_s)))
        && (Bits(attributes, 26, 26) == 1 ==> name == LowerBytesNullTerminated(PerfDomainName(domain_id(old_s)), 15))
        && guaranteed_perf_level == GuaranteedPerfLevel(domain_id(old_s)) || guaranteed_perf_level == 0
        && Bits(qos_capability_types, 31, 24) == 0
        && Bits(qos_capability_types, 15, 8) == 0
        && Bits(qos_capability_types, 23, 16) == SupportedOemQosCapabilityTypes(domain_id(old_s))
        && Bits(qos_capability_types, 7, 0) == SupportedArchQosCapabilityTypes(domain_id(old_s))
        && (qos_parent_id == 0xFFFFFFFF) == !HasQosParentDomain(domain_id(old_s))
        && HasQosParentDomain(domain_id(old_s)) ==> qos_parent_id == QosParentDomainId(domain_id(old_s))
    ))
}