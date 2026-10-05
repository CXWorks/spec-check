pub open spec fn performance_qos_capability_subtypes__3_5_6_6_spec(domain_id: UInt32, capability_type: UInt32, status: Int32, capability_subtypes: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidPerfDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (BitCount(capability_type[7:0]) > 1 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsValidQosCapabilityType(old_s, domain_id, capability_type) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> capability_subtypes == SupportedQosCapabilitySubtypes(new_s, domain_id, capability_type))
  && ((IsValidPerfDomain(old_s, domain_id) &&
       !(BitCount(capability_type[7:0]) > 1) &&
       IsValidQosCapabilityType(old_s, domain_id, capability_type))
    ==> ResultEqual(status, SUCCESS))
}