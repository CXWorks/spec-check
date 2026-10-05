pub open spec fn voltage_level_set__3_9_2_9_spec(domain_id: UInt32, flags: UInt32, voltage_level: Int32, status: Int32, old_s: S, new_s: S) -> bool {
  (!VoltageDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsSupportedVoltageLevel(old_s, domain_id, voltage_level) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsVoltageLevelSetRequestSupported(old_s, domain_id, flags, voltage_level) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMaySetVoltageLevel(old_s, calling_agent, domain_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 0, 0) == 0 ==> VoltageLevel(new_s, domain_id) == voltage_level)
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 0, 0) == 1 ==> VoltageLevelSetQueued(new_s, domain_id, voltage_level))
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 0, 0) == 1 ==> CompletesWithVoltageLevelSetCompleteMessage(new_s, domain_id))
  && ((VoltageDomainExists(old_s, domain_id) &&
       IsSupportedVoltageLevel(old_s, domain_id, voltage_level) &&
       IsVoltageLevelSetRequestSupported(old_s, domain_id, flags, voltage_level) &&
       AgentMaySetVoltageLevel(old_s, calling_agent, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> VoltageLevel(new_s, domain_id) == VoltageLevel(old_s, domain_id))
  && (result != SUCCESS
    ==> VoltageLevelSetQueued(new_s, domain_id, voltage_level) == false)
}