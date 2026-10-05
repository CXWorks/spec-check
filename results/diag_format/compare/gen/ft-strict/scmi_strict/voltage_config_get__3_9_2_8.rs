pub open spec fn voltage_config_get__3_9_2_8_spec(domain_id: uint32, status: int32, config: uint32, old_s: S, new_s: S) -> bool {
  (!IsValidVoltageDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsRequestSupported(old_s, 0x6, 0x17) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMayGetVoltageConfig(old_s, calling_agent, domain_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> Bits(config, 31, 4) == 0)
  && (ResultEqual(status, SUCCESS) ==> Bits(config, 3, 0) == VoltageDomainMode(new_s, domain_id))
  && ((IsValidVoltageDomain(old_s, domain_id) &&
       IsRequestSupported(old_s, 0x6, 0x17) &&
       AgentMayGetVoltageConfig(old_s, calling_agent, domain_id))
    ==> ResultEqual(status, SUCCESS))
}