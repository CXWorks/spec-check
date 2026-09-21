pub open spec fn 3.5.6.5_performance_domain_attributes_spec(domain_id: UInt32, attributes: UInt32, rate_limit: UInt32, sustained_freq: UInt32, sustained_perf_level: UInt32, name: [uint8; 16], guaranteed_perf_level: UInt32, qos_capability_types: UInt32, qos_parent_id: UInt32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Ok() ==> true)
  && (result.is_Err() ==> true)
}