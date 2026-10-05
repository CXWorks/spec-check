pub open spec fn reduce_sustained_performance_level__3_5_6_13_spec(domain_id: uint32, sustained_level: uint32, status: int32, old_s: S, new_s: S) -> bool {
  (!IsCommandSupported(old_s, REDUCE_SUSTAINED_PERFORMANCE_LEVEL) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!IsValidPerfDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!AgentPermittedToReduceSustainedLevel(old_s, agent, domain_id) ==> ResultEqual(status, DENIED))
  && (!IsInAllowedSustainedRange(old_s, domain_id, LevelOrIndex(old_s, domain_id, sustained_level)) ==> ResultEqual(status, OUT_OF_RANGE))
  && (result == SUCCESS ==> ResultEqual(status, SUCCESS))
  && (result == SUCCESS ==> ReducedSustainedLevelScheduled(new_s, domain_id, LevelOrIndex(new_s, domain_id, sustained_level)))
  && (result == SUCCESS ==> PerfDomainAttributes(new_s, domain_id).sustained_perf_level == PlatformSustainedPerfLevel(new_s, domain_id))
  && ((IsCommandSupported(old_s, REDUCE_SUSTAINED_PERFORMANCE_LEVEL) &&
       IsValidPerfDomain(old_s, domain_id) &&
       AgentPermittedToReduceSustainedLevel(old_s, agent, domain_id) &&
       IsInAllowedSustainedRange(old_s, domain_id, LevelOrIndex(old_s, domain_id, sustained_level)))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> PerfDomainAttributes(new_s, domain_id).sustained_perf_level == PerfDomainAttributes(old_s, domain_id).sustained_perf_level)
}