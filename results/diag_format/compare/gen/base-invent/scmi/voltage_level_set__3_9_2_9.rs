pub open spec fn voltage_level_set__3_9_2_9_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> !DomainExists(old_s, domain_id))
    && (result == INVALID_PARAMETERS ==> !VoltageLevelSupported(old_s, domain_id, voltage_level))
    && (result == NOT_SUPPORTED ==> !VoltageLevelSetSupported(old_s, domain_id))
    && (result == DENIED ==> !AgentAllowedToSetVoltage(old_s, domain_id))
    && (result == SUCCESS ==> VoltageLevelSet(old_s, domain_id, voltage_level, flags, new_s))
}