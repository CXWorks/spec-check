pub open spec fn voltage_config_get__3_9_2_8_spec(domain_id: u32, status: i32, config: u32, old_s: S, new_s: S) -> bool {
    (!IsValidVoltageDomain(old_s, domain_id) ==> status == NOT_FOUND)
    && ((IsValidVoltageDomain(old_s, domain_id) && !IsVoltageConfigGetSupported(old_s, domain_id)) ==> status == NOT_SUPPORTED)
    && ((IsValidVoltageDomain(old_s, domain_id) && IsVoltageConfigGetSupported(old_s, domain_id) && !IsAgentAllowedVoltageConfigGet(old_s, domain_id)) ==> status == DENIED)
    && (status == SUCCESS ==> (
        IsValidVoltageDomain(old_s, domain_id)
        && IsVoltageConfigGetSupported(old_s, domain_id)
        && IsAgentAllowedVoltageConfigGet(old_s, domain_id)
        && (config >> 4u32) == 0u32
        && (config & 0xFu32) == VoltageDomainMode(old_s, domain_id)
    ))
    && new_s == old_s
}
