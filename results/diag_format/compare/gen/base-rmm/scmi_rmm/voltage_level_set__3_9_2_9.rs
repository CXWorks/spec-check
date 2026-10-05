pub open spec fn voltage_level_set__3_9_2_9_spec(result: int, old_s: S, new_s: S) -> bool {
    (!VoltageDomainExists(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!VoltageLevelSupported(old_s, domain_id, voltage_level) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!RequestSupported(old_s, domain_id, flags, voltage_level) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMaySetVoltageLevel(old_s, caller, domain_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (
        (flags[0] == 0 ==> VoltageDomain(new_s, domain_id).voltage_level == voltage_level)
        && (flags[0] == 1 ==> CommandQueued(old_s, VOLTAGE_LEVEL_SET, domain_id, voltage_level))
    ))
}