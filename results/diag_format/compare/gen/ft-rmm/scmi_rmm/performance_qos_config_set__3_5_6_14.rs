pub open spec fn performance_qos_config_set__3_5_6_14_spec(domain_id: UInt32, capability: QosCapabilityDescriptor, flags: Bits32, qos_value: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (flags[4] == 0 && !IsValidPerfDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidQosCapability(old_s, domain_id, capability) ==> ResultEqual(status, NOT_FOUND))
  && (CountSetBits(capability[23:16]) > 1 || CountSetBits(capability[7:0]) > 1 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!AreValidQosConfigFlags(old_s, flags) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (flags[4:2] == 0 && !IsSupportedQosValue(old_s, domain_id, capability, qos_value) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!AgentMayConfigureQos(old_s, agent, domain_id, capability) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> flags[1] == 0 && flags[4:2] == 0 ⇒ QosValue(new_s, domain_id, capability) == qos_value)
  && (ResultEqual(status, SUCCESS) ==> flags[1] == 0 && flags[2] == 1 ⇒ QosValue(new_s, domain_id, capability) == PlatformDefaultQosValue(new_s, domain_id, capability))
  && (ResultEqual(status, SUCCESS) ==> flags[1] == 0 && flags[3] == 1 ⇒ ∀ d ∈ {domain_id} ∪ SiblingDomains(new_s, domain_id): QosValue(new_s, d, capability) == PlatformDefaultQosValue(new_s, d, capability))
  && (ResultEqual(status, SUCCESS) ==> flags[1] == 0 && flags[4] == 1 ⇒ ∀ d ∈ AllPerfDomains(new_s): QosValue(new_s, d, capability) == PlatformDefaultQosValue(new_s, d, capability))
  && (ResultEqual(status, SUCCESS) ==> flags[1] == 1 ⇒ QosConfigRequestQueued(new_s, domain_id, capability, flags, qos_value))
  && (ResultEqual(status, SUCCESS) ==> flags[1] == 1 && flags[0] == 0 ⇒ DelayedResponseSent(new_s, PERFORMANCE_QOS_CONFIG_COMPLETE))
  && ((!(flags[4] == 0 && !IsValidPerfDomain(old_s, domain_id)) &&
       IsValidQosCapability(old_s, domain_id, capability) &&
       !(CountSetBits(capability[23:16]) > 1 || CountSetBits(capability[7:0]) > 1) &&
       AreValidQosConfigFlags(old_s, flags) &&
       !(flags[4:2] == 0 && !IsSupportedQosValue(old_s, domain_id, capability, qos_value)) &&
       AgentMayConfigureQos(old_s, agent, domain_id, capability))
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
    ==> QosConfigRequestQueued(new_s, domain_id, capability, flags, qos_value) == false)
  && (result != SUCCESS
    ==> DelayedResponseSent(new_s, PERFORMANCE_QOS_CONFIG_COMPLETE) == false)
}