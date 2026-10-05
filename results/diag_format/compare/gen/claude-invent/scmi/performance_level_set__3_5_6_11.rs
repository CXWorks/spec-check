pub open spec fn performance_level_set__3_5_6_11_spec(domain_id: UInt32, performance_level: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidPerformanceDomain(old_s, domain_id) ==> (status == NOT_FOUND && new_s == old_s))
    && ((IsValidPerformanceDomain(old_s, domain_id)
        && !AgentPermittedToSetPerformanceLevel(old_s, domain_id))
        ==> (status == DENIED && new_s == old_s))
    && ((IsValidPerformanceDomain(old_s, domain_id)
        && AgentPermittedToSetPerformanceLevel(old_s, domain_id)
        && !PerformanceLevelInAllowedRange(old_s, domain_id, performance_level))
        ==> (status == OUT_OF_RANGE && new_s == old_s))
    && ((IsValidPerformanceDomain(old_s, domain_id)
        && AgentPermittedToSetPerformanceLevel(old_s, domain_id)
        && PerformanceLevelInAllowedRange(old_s, domain_id, performance_level))
        ==> (status == SUCCESS
            && PerformanceLevelRequestScheduled(new_s, domain_id, performance_level)))
}
