pub open spec fn reduce_sustained_performance_level__3_5_6_13_spec(result: int32, old_s: S, new_s: S) -> bool {
    (!IsCommandSupported(REDUCE_SUSTAINED_PERFORMANCE_LEVEL) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidPerfDomain(domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (PerfDomainAttributes(domain_id).attributes[23] == 1 ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMayReduceSustainedLevel(agent, domain_id) ==> ResultEqual(result, DENIED))
    && (!IsInAllowedRange(domain_id, sustained_level) ==> ResultEqual(result, OUT_OF_RANGE))
    && (ResultEqual(result, SUCCESS) ==> PlatformHasAcceptedAndScheduled(domain_id, sustained_level))
    && (ResultEqual(result, SUCCESS) ==> PerfDomainAttributes(domain_id).sustained_perf_level == old(PerfDomainAttributes(domain_id).sustained_perf_level))
}