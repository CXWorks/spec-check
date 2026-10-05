pub open spec fn voltage_level_set__3_9_2_9_spec(domain_id: uint32, flags: uint32, voltage_level: int32, status: int32, old_s: S, new_s: S) -> bool {
  (!VoltageDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!VoltageLevelSupported(old_s, domain_id, voltage_level) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!RequestSupported(old_s, domain_id, flags, voltage_level) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMaySetVoltageLevel(old_s, caller, domain_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> status == SUCCESS)
  && (flags[0] == 0 ==> VoltageDomain(new_s, domain_id).voltage_level == voltage_level)
  && (flags[0] == 1 ==> CommandQueued(new_s, VOLTAGE_LEVEL_SET, domain_id, voltage_level))
  && ((VoltageDomainExists(old_s, domain_id) &&
       VoltageLevelSupported(old_s, domain_id, voltage_level) &&
       RequestSupported(old_s, domain_id, flags, voltage_level) &&
       AgentMaySetVoltageLevel(old_s, caller, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> VoltageDomain(new_s, domain_id).voltage_level == VoltageDomain(old_s, domain_id).voltage_level)
}