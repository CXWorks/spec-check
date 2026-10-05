pub open spec fn performance_qos_attributes__3_5_6_7_spec(domain_id: uint32, capability: uint32, status: int32, qos_attribute_1: uint32, name: [uint8; 16], old_s: S, new_s: S) -> bool {
  (!IsValidPerformanceDomain(old_s, domain_id) ==> ResultEqual(Result::Err(NOT_FOUND), status))
  && (!IsValidQosCapabilityOfDomain(old_s, domain_id, capability) ==> ResultEqual(Result::Err(NOT_FOUND), status))
  && (CountQosCapabilityTypeBits(old_s, capability) > 1 ==> ResultEqual(Result::Err(INVALID_PARAMETERS), status))
  && (CountQosCapabilitySubtypeBits(old_s, capability) > 1 ==> ResultEqual(Result::Err(INVALID_PARAMETERS), status))
  && (ResultEqual(Result::Ok(()), status) ==> qos_attribute_1 == QosCapabilityFirstAttribute(old_s, domain_id, capability))
  && (ResultEqual(Result::Ok(()), status) ==> name == QosCapabilityName(old_s, domain_id, capability))
  && ((IsValidPerformanceDomain(old_s, domain_id) &&
       IsValidQosCapabilityOfDomain(old_s, domain_id, capability) &&
       !(CountQosCapabilityTypeBits(old_s, capability) > 1) &&
       !(CountQosCapabilitySubtypeBits(old_s, capability) > 1))
    ==> ResultEqual(Result::Ok(()), status))
}