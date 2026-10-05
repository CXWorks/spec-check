pub open spec fn performance_qos_config_get__3_5_6_15_spec(domain_id: UInt32, capability: UInt32, status: Int32, qos_value: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidPerformanceDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidQosCapability(old_s, domain_id, capability) ==> ResultEqual(status, NOT_FOUND))
  && (QosCapabilityTypeBitCount(old_s, capability) > 1 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (QosCapabilitySubtypeBitCount(old_s, capability) > 1 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> qos_value == QosConfiguration(new_s, domain_id, capability))
  && ((IsValidPerformanceDomain(old_s, domain_id) &&
       IsValidQosCapability(old_s, domain_id, capability) &&
       !(QosCapabilityTypeBitCount(old_s, capability) > 1) &&
       !(QosCapabilitySubtypeBitCount(old_s, capability) > 1))
    ==> ResultEqual(status, SUCCESS))
}