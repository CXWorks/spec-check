pub open spec fn voltage_config_get__3_9_2_8_spec(domain_id: UInt32, status: Int32, config: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidVoltageDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsRequestSupported(old_s) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMayGetVoltageConfig(old_s, calling_agent, domain_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> config[31:4] == 0)
  && (ResultEqual(status, SUCCESS) ==> config[3:0] == VoltageDomain(new_s, domain_id).mode)
  && ((IsValidVoltageDomain(old_s, domain_id) &&
       IsRequestSupported(old_s) &&
       AgentMayGetVoltageConfig(old_s, calling_agent, domain_id))
    ==> ResultEqual(status, SUCCESS))
}