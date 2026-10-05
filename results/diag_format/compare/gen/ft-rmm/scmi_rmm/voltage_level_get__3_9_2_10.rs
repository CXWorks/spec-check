pub open spec fn voltage_level_get__3_9_2_10_spec(domain_id: uint32, status: int32, voltage_level: int32, old_s: S, new_s: S) -> bool {
  (!IsValidVoltageDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsRequestSupported(old_s, domain_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMayGetVoltageLevel(old_s, caller, domain_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> voltage_level == VoltageDomain(new_s, domain_id).level)
  && ((IsValidVoltageDomain(old_s, domain_id) &&
       IsRequestSupported(old_s, domain_id) &&
       AgentMayGetVoltageLevel(old_s, caller, domain_id))
    ==> ResultEqual(status, SUCCESS))
}