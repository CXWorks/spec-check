pub open spec fn voltage_level_get__3_9_2_10_spec(domain_id: UInt32, status: Int32, voltage_level: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidVoltageDomain(old_s, domain_id) ==> ResultEqual((status, voltage_level), (NOT_FOUND, 0)))
  && (!IsRequestSupported(old_s, domain_id) ==> ResultEqual((status, voltage_level), (NOT_SUPPORTED, 0)))
  && (!AgentMayGetVoltageLevel(old_s, calling_agent, domain_id) ==> ResultEqual((status, voltage_level), (DENIED, 0)))
  && (ResultEqual((status, voltage_level), (SUCCESS, _)) ==> ResultEqual((status, voltage_level), (SUCCESS, VoltageLevelDuringCommand(new_s, domain_id))))
  && ((!(IsValidVoltageDomain(old_s, domain_id)) &&
       IsRequestSupported(old_s, domain_id) &&
       AgentMayGetVoltageLevel(old_s, calling_agent, domain_id))
    ==> ResultEqual((status, voltage_level), (SUCCESS, VoltageLevelDuringCommand(new_s, domain_id))))
}