pub open spec fn performance_level_get__3_5_6_12_spec(domain_id: u32, status: i32, performance_level: u32, old_s: S, new_s: S) -> bool {
    (!IsValidPerformanceDomain(old_s, domain_id) ==> status == NOT_FOUND)
    && (status == SUCCESS ==> (IsValidPerformanceDomain(old_s, domain_id) && performance_level == PerformanceDomainCurrentLevel(old_s, domain_id)))
    && (new_s == old_s)
}
