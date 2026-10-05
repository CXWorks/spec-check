pub open spec fn reduce_sustained_performance_level__3_5_6_13_spec(domain_id: uint32, sustained_level: uint32, status: int32, old_s: S, new_s: S) -> bool {
  (!IsCommandSupported(old_s, REDUCE_SUSTAINED_PERFORMANCE_LEVEL) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!IsValidPerfDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (PerfDomainAttributes(old_s, domain_id).attributes[23] == 1 ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMayReduceSustainedLevel(old_s, agent, domain_id) ==> ResultEqual(status, DENIED))
  && (!IsInAllowedRange(old_s, domain_id, sustained_level) ==> ResultEqual(status, OUT_OF_RANGE))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> PlatformHasAcceptedAndScheduled(new_s, domain_id, sustained_level))
  && (ResultEqual(status, SUCCESS) ==> PerfDomainAttributes(new_s, domain_id).sustained_perf_level == PerfDomainAttributes(new_s, domain_id).sustained_perf_level)
  && ((IsCommandSupported(old_s, REDUCE_SUSTAINED_PERFORMANCE_LEVEL) &&
       IsValidPerfDomain(old_s, domain_id) &&
       !(PerfDomainAttributes(old_s, domain_id).attributes[23] == 1) &&
       AgentMayReduceSustainedLevel(old_s, agent, domain_id) &&
       IsInAllowedRange(old_s, domain_id, sustained_level))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> PerfDomainAttributes(new_s, domain_id).sustained_perf_level == PerfDomainAttributes(old_s, domain_id).sustained_perf_level)
}