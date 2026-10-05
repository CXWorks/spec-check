pub open spec fn voltage_level_set__3_9_2_9_spec(domain_id: u32, flags: u32, voltage_level: i32, status: i32, old_s: S, new_s: S) -> bool {
    (!VoltageDomainExists(old_s, domain_id) ==> (status == NOT_FOUND && new_s == old_s))
    && ((VoltageDomainExists(old_s, domain_id) && !VoltageLevelSetRequestSupported(old_s, domain_id, flags)) ==> (status == NOT_SUPPORTED || status == DENIED || status == INVALID_PARAMETERS))
    && ((VoltageDomainExists(old_s, domain_id) && VoltageLevelSetRequestSupported(old_s, domain_id, flags) && !AgentAllowedToSetVoltageLevel(old_s, domain_id) && VoltageLevelSupported(old_s, domain_id, voltage_level)) ==> (status == DENIED && new_s == old_s))
    && ((VoltageDomainExists(old_s, domain_id) && VoltageLevelSetRequestSupported(old_s, domain_id, flags) && AgentAllowedToSetVoltageLevel(old_s, domain_id) && !VoltageLevelSupported(old_s, domain_id, voltage_level)) ==> (status == INVALID_PARAMETERS && new_s == old_s))
    && ((VoltageDomainExists(old_s, domain_id) && VoltageLevelSetRequestSupported(old_s, domain_id, flags) && !AgentAllowedToSetVoltageLevel(old_s, domain_id) && !VoltageLevelSupported(old_s, domain_id, voltage_level)) ==> ((status == DENIED || status == INVALID_PARAMETERS) && new_s == old_s))
    && ((status == NOT_FOUND || status == INVALID_PARAMETERS || status == NOT_SUPPORTED || status == DENIED) ==> new_s == old_s)
    && ((VoltageDomainExists(old_s, domain_id) && VoltageLevelSetRequestSupported(old_s, domain_id, flags) && AgentAllowedToSetVoltageLevel(old_s, domain_id) && VoltageLevelSupported(old_s, domain_id, voltage_level)) ==> (
        status == SUCCESS
        && (((flags & 1u32) == 0u32) ==> VoltageLevel(new_s, domain_id) == voltage_level)
        && (((flags & 1u32) == 1u32) ==> VoltageLevelSetQueued(new_s, domain_id, voltage_level))
        && (forall|d: u32| d != domain_id ==> VoltageLevel(new_s, d) == VoltageLevel(old_s, d))
    ))
}
