pub open spec fn voltage_config_get__3_9_2_8_spec(result: Int32, config: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidVoltageDomain(domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsRequestSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMayGetVoltageConfig(calling_agent(old_s), domain_id(old_s)) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (config[31:4] == 0))
    && (ResultEqual(result, SUCCESS) ==> (config[3:0] == VoltageDomain(domain_id(old_s)).mode))
}