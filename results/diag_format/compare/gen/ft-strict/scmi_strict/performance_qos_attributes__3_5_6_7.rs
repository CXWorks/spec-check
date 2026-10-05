pub open spec fn performance_qos_attributes__3_5_6_7_spec(domain_id: UInt32, capability: UInt32, status: Int32, qos_attribute_1: UInt32, name: [UInt8; 16], old_s: S, new_s: S) -> bool {
  (!IsValidPerformanceDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidQosCapability(old_s, domain_id, capability) ==> ResultEqual(status, NOT_FOUND))
  && (QosCapabilityTypeBitCount(old_s, capability) > 1 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (QosCapabilitySubtypeBitCount(old_s, capability) > 1 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> qos_attribute_1 == QosCapabilityFirstAttribute(old_s, domain_id, capability))
  && (ResultEqual(status, SUCCESS) ==> QosCapabilityNameEqual(new_s, name, domain_id, capability))
  && (ResultEqual(status, SUCCESS) ==> IsNullTerminatedAscii(new_s, name, 16))
  && ((IsValidPerformanceDomain(old_s, domain_id) &&
       IsValidQosCapability(old_s, domain_id, capability) &&
       !(QosCapabilityTypeBitCount(old_s, capability) > 1) &&
       !(QosCapabilitySubtypeBitCount(old_s, capability) > 1))
    ==> ResultEqual(status, SUCCESS))
}