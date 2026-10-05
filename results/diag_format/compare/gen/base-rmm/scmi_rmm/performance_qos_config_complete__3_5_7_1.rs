pub open spec fn performance_qos_config_complete__3_5_7_1_spec(result: RsiCommandReturnCode, status: int, domain_id: UInt32, capability: UInt32, flags: UInt32, qos_value: UInt32, old_s: S, new_s: S) -> bool {
    (ResultEqual(result, RSI_SUCCESS) ==> (status == RSI_SUCCESS as int))
    && (flags[31:5] == 0)
    && (flags[1:0] == 0)
    && (flags[4] == 1 ==> (domain_id == 0xFFFFFFFF && AllDomainsQosAtPlatformDefault(capability)))
    && (flags[3] == 1 ==> DomainAndSiblingsQosAtPlatformDefault(domain_id, capability))
    && (flags[2] == 1 ==> QosAtPlatformDefault(domain_id, capability))
    && (flags[4] == 0 && flags[3] == 0 && flags[2] == 0 ==> QosValue(domain_id, capability) == qos_value)
}