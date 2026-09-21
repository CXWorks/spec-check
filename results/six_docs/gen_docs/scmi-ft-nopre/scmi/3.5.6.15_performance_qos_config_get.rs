pub open spec fn 3.5.6.15_performance_qos_config_get_spec(domain_id: UInt32, capability: UInt32, result: RsiCommandReturnCode, qos_value: UInt32, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> qos_value == GetPerformanceDomainQosConfig(new_s, domain_id, capability))
}