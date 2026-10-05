pub open spec fn performance_qos_config_get__3_5_6_15_spec(domain_id: UInt32, capability: UInt32, result: RsiCommandReturnCode, qos_value: UInt32, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> qos_value == GetQosConfig(new_s, domain_id, capability))
}