pub open spec fn performance_domain_attributes__3_5_6_5_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> !DomainExists(old_s, domain_id))
    && (result == SUCCESS ==> DomainExists(old_s, domain_id))
    && (result == SUCCESS ==> (new_s.attributes == old_s.attributes))
    && (result == SUCCESS ==> (new_s.rate_limit == old_s.rate_limit))
    && (result == SUCCESS ==> (new_s.sustained_freq == old_s.sustained_freq))
    && (result == SUCCESS ==> (new_s.sustained_perf_level == old_s.sustained_perf_level))
    && (result == SUCCESS ==> (new_s.name == old_s.name))
    && (result == SUCCESS ==> (new_s.guaranteed_perf_level == old_s.guaranteed_perf_level))
    && (result == SUCCESS ==> (new_s.qos_capability_types == old_s.qos_capability_types))
    && (result == SUCCESS ==> (new_s.qos_parent_id == old_s.qos_parent_id))
}