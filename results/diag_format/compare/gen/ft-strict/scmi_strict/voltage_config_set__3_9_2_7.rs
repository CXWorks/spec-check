pub open spec fn voltage_config_set__3_9_2_7_spec(domain_id: UInt32, config: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!VoltageDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!VoltageDomainSupportsMode(old_s, domain_id, (config >> 3 & 7)) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsRequestSupported(old_s, VOLTAGE_CONFIG_SET) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMaySetVoltageConfig(old_s, CallingAgent(old_s), domain_id) ==> ResultEqual(status, DENIED))
  && (result == SUCCESS ==> VoltageDomain(new_s, domain_id).mode == (config >> 3 & 7))
  && ((!(VoltageDomainExists(old_s, domain_id)) &&
       VoltageDomainSupportsMode(old_s, domain_id, (config >> 3 & 7)) &&
       IsRequestSupported(old_s, VOLTAGE_CONFIG_SET) &&
       AgentMaySetVoltageConfig(old_s, CallingAgent(old_s), domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> VoltageDomain(new_s, domain_id).mode == VoltageDomain(old_s, domain_id).mode)
}