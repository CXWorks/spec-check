pub open spec fn voltage_config_set__3_9_2_7_spec(domain_id: u32, config: u32, status: i32, old_s: S, new_s: S) -> bool {
    (!VoltageDomainExists(old_s, domain_id) ==> status == NOT_FOUND)
    && (status == NOT_FOUND ==> !VoltageDomainExists(old_s, domain_id))
    && (status == INVALID_PARAMETERS ==> (VoltageDomainExists(old_s, domain_id) && !VoltageConfigSupported(old_s, domain_id, config & 0xF)))
    && (status == DENIED ==> (VoltageDomainExists(old_s, domain_id) && !AgentMaySetVoltageConfig(old_s, domain_id)))
    && (status == NOT_SUPPORTED ==> !VoltageConfigSetRequestSupported(old_s, domain_id, config))
    && (status != SUCCESS ==> new_s == old_s)
    && ((VoltageDomainExists(old_s, domain_id)
        && (config >> 4u32) == 0
        && VoltageConfigSupported(old_s, domain_id, config & 0xF)
        && AgentMaySetVoltageConfig(old_s, domain_id)
        && VoltageConfigSetRequestSupported(old_s, domain_id, config))
        ==> status == SUCCESS)
    && (status == SUCCESS ==> (
        VoltageDomainExists(old_s, domain_id)
        && VoltageConfigSupported(old_s, domain_id, config & 0xF)
        && AgentMaySetVoltageConfig(old_s, domain_id)
        && VoltageConfigSetRequestSupported(old_s, domain_id, config)
        && VoltageDomainExists(new_s, domain_id)
        && VoltageDomainMode(new_s, domain_id) == (config & 0xF)
        && (forall|d: u32| d != domain_id ==> VoltageDomainMode(new_s, d) == VoltageDomainMode(old_s, d))
        && (forall|d: u32| VoltageDomainExists(new_s, d) == VoltageDomainExists(old_s, d))
    ))
}
