pub open spec fn 3.5.6.9_performance_limits_set_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (old_s.performance_domains[old_s.domain_id as usize].exists() && (old_s.performance_domains[old_s.domain_id as usize].range_max == 0 || old_s.performance_domains[old_s.domain_id as usize].range_min == 0 || old_s.performance_domains[old_s.domain_id as usize].range_max != old_s.performance_domains[old_s.domain_id as usize].range_min)))
    && (result == 1 ==> !old_s.performance_domains[old_s.domain_id as usize].exists())
    && (result == 2 ==> (old_s.performance_domains[old_s.domain_id as usize].exists() && (old_s.performance_domains[old_s.domain_id as usize].range_max == 0 && old_s.performance_domains[old_s.domain_id as usize].range_min == 0 || (old_s.performance_domains[old_s.domain_id as usize].range_max as int) < (old_s.performance_domains[old_s.domain_id as usize].range_min as int))))
    && (result == 3 ==> (old_s.performance_domains[old_s.domain_id as usize].exists() && !old_s.performance_domains[old_s.domain_id as usize].is_agent_permitted_to_change_limits(old_s.agent_id)))
    && (result != 0 && result != 1 && result != 2 && result != 3 ==> true)
}