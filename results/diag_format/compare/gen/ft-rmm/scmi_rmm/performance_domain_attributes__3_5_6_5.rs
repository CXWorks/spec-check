pub open spec fn performance_domain_attributes__3_5_6_5_spec(domain_id: uint32, status: int32, attributes: uint32, rate_limit: uint32, sustained_freq: uint32, sustained_perf_level: uint32, name: [uint8; 16], guaranteed_perf_level: uint32, qos_capability_types: uint32, qos_parent_id: uint32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!IsValidPerformanceDomain(old_s, domain_id) ==> result == RSI_ERROR_NOT_FOUND)
  && (result == RSI_SUCCESS ==> status == 0)
  && (result == RSI_SUCCESS ==> (attributes & 0x3FFFFF) == 0)
  && (result == RSI_SUCCESS ==> (attributes & (1 << 23)) == 1 ==> (attributes & 0xF0000000) == 0)
  && (result == RSI_SUCCESS ==> (attributes & (1 << 23)) == 1 ==> (attributes & (1 << 22)) == 0)
  && (result == RSI_SUCCESS ==> (rate_limit & 0xFFF00000) == 0)
  && (result == RSI_SUCCESS ==> (qos_capability_types & 0xF0000000) == 0 && (qos_capability_types & 0xF00) == 0)
  && ((IsValidPerformanceDomain(old_s, domain_id))
    ==> result == RSI_SUCCESS)
}