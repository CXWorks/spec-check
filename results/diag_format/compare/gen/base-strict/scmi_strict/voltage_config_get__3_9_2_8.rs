pub open spec fn voltage_config_get__3_9_2_8_spec(result: int32, config: uint32, old_s: S, new_s: S) -> bool {
    (!IsValidVoltageDomain(domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsRequestSupported(message_id(old_s), protocol_id(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMayGetVoltageConfig(calling_agent(old_s), domain_id(old_s)) ==> ResultEqual(result, DENIED))
    && (IsValidVoltageDomain(domain_id(old_s)) && IsRequestSupported(message_id(old_s)) && AgentMayGetVoltageConfig(calling_agent(old_s), domain_id(old_s)) ==> ResultEqual(result, SUCCESS))
    && (ResultEqual(result, SUCCESS) ==> Bits(config, 31, 4) == 0)
    && (ResultEqual(result, SUCCESS) ==> Bits(config, 3, 0) == VoltageDomainMode(domain_id(old_s)))
    && (old_s == new_s)
}