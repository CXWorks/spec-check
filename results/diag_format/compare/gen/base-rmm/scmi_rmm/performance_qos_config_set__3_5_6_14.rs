pub open spec fn performance_qos_config_set__3_5_6_14_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidPerfDomain(old_s, req.domain_id) && req.flags[4] == 0 ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidQosCapability(old_s, req.domain_id, req.capability) ==> ResultEqual(result, NOT_FOUND))
    && (CountSetBits(req.capability[23:16]) > 1 || CountSetBits(req.capability[7:0]) > 1 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!AreValidQosConfigFlags(old_s, req.flags) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (req.flags[4:2] == 0 && !IsSupportedQosValue(old_s, req.domain_id, req.capability, req.qos_value) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!AgentMayConfigureQos(old_s, req.agent, req.domain_id, req.capability) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (req.flags[1] == 0 && req.flags[4:2] == 0 ==> QosValue(new_s, req.domain_id, req.capability) == req.qos_value))
    && (ResultEqual(result, SUCCESS) ==> (req.flags[1] == 0 && req.flags[2] == 1 ==> QosValue(new_s, req.domain_id, req.capability) == PlatformDefaultQosValue(old_s, req.domain_id, req.capability)))
    && (ResultEqual(result, SUCCESS) ==> (req.flags[1] == 0 && req.flags[3] == 1 ==> forall d in SiblingDomains(old_s, req.domain_id) + {req.domain_id}: QosValue(new_s, d, req.capability) == PlatformDefaultQosValue(old_s, d, req.capability)))
    && (ResultEqual(result, SUCCESS) ==> (req.flags[1] == 0 && req.flags[4] == 1 ==> forall d in AllPerfDomains(old_s): QosValue(new_s, d, req.capability) == PlatformDefaultQosValue(old_s, d, req.capability)))
    && (ResultEqual(result, SUCCESS) ==> (req.flags[1] == 1 ==> QosConfigRequestQueued(old_s, req.domain_id, req.capability, req.flags, req.qos_value)))
    && (ResultEqual(result, SUCCESS) ==> (req.flags[1] == 1 && req.flags[0] == 0 ==> DelayedResponseSent(old_s, PERFORMANCE_QOS_CONFIG_COMPLETE)))
}