pub open spec fn voltage_level_get__3_9_2_10_spec(domain_id: u32, status: i32, voltage_level: i32, old_s: S, new_s: S) -> bool {
    (!IsValidVoltageDomain(old_s, domain_id) ==> status == NOT_FOUND)
    && (status == NOT_FOUND ==> !IsValidVoltageDomain(old_s, domain_id))
    && (status == DENIED ==> (IsValidVoltageDomain(old_s, domain_id) && !AgentAllowedToGetVoltageLevel(old_s, domain_id)))
    && (status == SUCCESS ==> (IsValidVoltageDomain(old_s, domain_id)
        && AgentAllowedToGetVoltageLevel(old_s, domain_id)
        && voltage_level == VoltageDomainLevel(old_s, domain_id)))
    && (new_s == old_s)
}
