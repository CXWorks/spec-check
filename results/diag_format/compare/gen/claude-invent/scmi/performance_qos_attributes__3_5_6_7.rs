pub open spec fn performance_qos_attributes__3_5_6_7_spec(domain_id: u32, capability: u32, status: i32, qos_attribute_1: u32, name: Seq<u8>, old_s: S, new_s: S) -> bool {
    (QosCapabilityHasMultipleTypeOrSubtypeBits(capability) ==> status == INVALID_PARAMETERS)
    && ((!QosCapabilityHasMultipleTypeOrSubtypeBits(capability)
        && (!IsValidPerformanceDomain(old_s, domain_id)
            || !IsValidQosCapabilityOfDomain(old_s, domain_id, capability))) ==> status == NOT_FOUND)
    && ((!QosCapabilityHasMultipleTypeOrSubtypeBits(capability)
        && IsValidPerformanceDomain(old_s, domain_id)
        && IsValidQosCapabilityOfDomain(old_s, domain_id, capability)) ==> (
            status == SUCCESS
            && qos_attribute_1 == PerformanceQosAttribute1(old_s, domain_id, capability)
            && name.len() == 16
            && name == PerformanceQosCapabilityName(old_s, domain_id, capability)
        ))
    && new_s == old_s
}
