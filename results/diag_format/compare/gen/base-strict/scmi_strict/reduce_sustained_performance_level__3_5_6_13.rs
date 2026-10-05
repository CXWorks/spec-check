pub open spec fn reduce_sustained_performance_level__3_5_6_13_spec(
    result: int32,
    old_s: S,
    new_s: S,
    domain_id: uint32,
    sustained_level: uint32,
) -> bool {
    (!IsCommandSupported(REDUCE_SUSTAINED_PERFORMANCE_LEVEL) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidPerfDomain(domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!AgentPermittedToReduceSustainedLevel(agent, domain_id) ==> ResultEqual(result, DENIED))
    && (!IsInAllowedSustainedRange(domain_id, LevelOrIndex(domain_id, sustained_level)) ==> ResultEqual(result, OUT_OF_RANGE))
    && (ResultEqual(result, SUCCESS) ==> ReducedSustainedLevelScheduled(domain_id, LevelOrIndex(domain_id, sustained_level)))
    && (ResultEqual(result, SUCCESS) ==> PerfDomainAttributes(domain_id).sustained_perf_level == PlatformSustainedPerfLevel(domain_id))
    && (ResultEqual(result, SUCCESS) ==> new_s.PerfDomain(domain_id).provisioned_sustained_level == old_s.PerfDomain(domain_id).provisioned_sustained_level)
}