pub open spec fn voltage_config_set__3_9_2_7_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!VoltageDomainExists(domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!VoltageDomainSupportsMode(domain_id(old_s), Bits(config(old_s), 3, 0)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsRequestSupported(VOLTAGE_CONFIG_SET) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMaySetVoltageConfig(CallingAgent(), domain_id(old_s)) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> VoltageDomain(domain_id(old_s)).mode == Bits(config(old_s), 3, 0))
}