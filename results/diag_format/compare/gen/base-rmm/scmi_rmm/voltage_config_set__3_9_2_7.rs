pub open spec fn voltage_config_set__3_9_2_7_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!VoltageDomainExists(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!VoltageDomainSupportsConfig(old_s, domain_id, config) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsRequestSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMaySetVoltageConfig(caller, domain_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> VoltageDomain(new_s, domain_id).mode == config.mode)
}