pub open spec fn performance_qos_capability_subtypes__3_5_6_6_spec(domain_id: UInt32, capability_type: UInt32, status: Int32, capability_subtypes: UInt32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!IsValidPerfDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
  && (!IsValidQosCapabilityType(old_s, domain_id, capability_type) ==> ResultEqual(result, NOT_FOUND))
  && (PopCount(Bits(capability_type, 7, 0)) > 1 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result.is_Ok() ==> ResultEqual(result, SUCCESS))
  && (result.is_Ok() ==> Bits(capability_subtypes, 31, 24) == 0)
  && (result.is_Ok() ==> Bits(capability_subtypes, 23, 16) == SupportedOemQosSubtypes(old_s, domain_id, capability_type))
  && (result.is_Ok() ==> Bits(capability_subtypes, 15, 8) == 0)
  && (result.is_Ok() ==> Bits(capability_subtypes, 7, 0) == SupportedArchitectedQosSubtypes(old_s, domain_id, capability_type))
  && ((IsValidPerfDomain(old_s, domain_id) &&
       IsValidQosCapabilityType(old_s, domain_id, capability_type) &&
       !(PopCount(Bits(capability_type, 7, 0)) > 1))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Bits(capability_subtypes, 31, 24) == 0)
  && (result.is_Err()
    ==> Bits(capability_subtypes, 23, 16) == 0)
  && (result.is_Err()
    ==> Bits(capability_subtypes, 15, 8) == 0)
  && (result.is_Err()
    ==> Bits(capability_subtypes, 7, 0) == 0)
}