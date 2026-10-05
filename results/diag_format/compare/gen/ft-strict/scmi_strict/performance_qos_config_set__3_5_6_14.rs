pub open spec fn performance_qos_config_set__3_5_6_14_spec(domain_id: UInt32, capability: UInt32, flags: UInt32, qos_value: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  ((flags & 0xF) == 0 && !IsValidPerfDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidQosCapability(old_s, domain_id, capability) ==> ResultEqual(status, NOT_FOUND))
  && (PopCount(Bits(capability, 23, 16)) > 1 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (PopCount(Bits(capability, 7, 0)) > 1 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsValidQosConfigFlags(old_s, flags) ==> ResultEqual(status, INVALID_PARAMETERS))
  && ((flags & 0xC) == 0 && !IsSupportedQosValue(old_s, domain_id, capability, qos_value) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!CallerMayConfigureQos(old_s, domain_id, capability) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> QosValue(new_s, domain_id, capability) == qos_value)
  && (ResultEqual(status, SUCCESS) && (flags & 0x2) == 0 && (flags & 0xC) == 0 ==> QosValue(new_s, domain_id, capability) == PlatformDefaultQos(new_s, domain_id, capability))
  && (ResultEqual(status, SUCCESS) && (flags & 0x2) == 0 && (flags & 0x4) == 1 ==> (forall d: PerfDomain, (d == domain_id || IsSiblingDomain(new_s, d, domain_id)) ==> QosValue(new_s, d, capability) == PlatformDefaultQos(new_s, d, capability)))
  && (ResultEqual(status, SUCCESS) && (flags & 0x8) == 0 && (flags & 0x8) == 1 ==> (forall d: PerfDomain, QosValue(new_s, d, capability) == PlatformDefaultQos(new_s, d, capability)))
  && (ResultEqual(status, SUCCESS) && (flags & 0x2) == 1 ==> QosConfigRequestQueued(new_s, domain_id, capability, flags, qos_value))
  && (ResultEqual(status, SUCCESS) && (flags & 0x2) == 1 && (flags & 0x1) == 0 ==> CompletesWithDelayedResponse(new_s, PERFORMANCE_QOS_CONFIG_COMPLETE))
  && (ResultEqual(status, SUCCESS) && (flags & 0x2) == 1 && (flags & 0x1) == 1 ==> !SendsDelayedResponse(new_s, PERFORMANCE_QOS_CONFIG_COMPLETE))
  && ((!( (flags & 0xF) == 0 && !IsValidPerfDomain(old_s, domain_id)) &&
       IsValidQosCapability(old_s, domain_id, capability) &&
       !(PopCount(Bits(capability, 23, 16)) > 1) &&
       !(PopCount(Bits(capability, 7, 0)) > 1) &&
       IsValidQosConfigFlags(old_s, flags) &&
       !((flags & 0xC) == 0 && !IsSupportedQosValue(old_s, domain_id, capability, qos_value)) &&
       CallerMayConfigureQos(old_s, domain_id, capability))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> QosValue(new_s, domain_id, capability) == QosValue(old_s, domain_id, capability))
  && (result != SUCCESS
    ==> QosValue(new_s, domain_id, capability) == QosValue(old_s, domain_id, capability))
  && (result != SUCCESS
    ==> QosValue(new_s, domain_id, capability) == QosValue(old_s, domain_id, capability))
  && (result != SUCCESS
    ==> QosValue(new_s, domain_id, capability) == QosValue(old_s, domain_id, capability))
  && (result != SUCCESS
    ==> QosValue(new_s, domain_id, capability) == QosValue(old_s, domain_id, capability))
  && (result != SUCCESS
    ==> QosValue(new_s, domain_id, capability) == QosValue(old_s, domain_id, capability))
  && (result != SUCCESS
    ==> QosValue(new_s, domain_id, capability) == QosValue(old_s, domain_id, capability))
  && (result != SUCCESS
    ==> QosValue(new_s, domain_id, capability) == QosValue(old_s, domain_id, capability))
  && (result != SUCCESS
    ==> QosValue(new_s, domain_id, capability) == QosValue(old_s, domain_id, capability))
  && (result != SUCCESS
    ==> QosValue(new_s, domain_id, capability) == QosValue(old_s, domain_id, capability))
}