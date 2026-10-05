pub open spec fn voltage_config_set__3_9_2_7_spec(domain_id: UInt32, config: [UInt32; 2], status: Int32, old_s: S, new_s: S) -> bool {
  (!VoltageDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!VoltageDomainSupportsConfig(old_s, domain_id, config) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsRequestSupported(old_s) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMaySetVoltageConfig(old_s, caller, domain_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> VoltageDomain(new_s, domain_id).mode == config[1])
  && ((VoltageDomainExists(old_s, domain_id) &&
       VoltageDomainSupportsConfig(old_s, domain_id, config) &&
       IsRequestSupported(old_s) &&
       AgentMaySetVoltageConfig(old_s, caller, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> VoltageDomain(new_s, domain_id).mode == VoltageDomain(old_s, domain_id).mode)
}