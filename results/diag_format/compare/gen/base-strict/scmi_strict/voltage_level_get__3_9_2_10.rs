pub open spec fn voltage_level_get__3_9_2_10_spec(result: Int32, voltage_level: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidVoltageDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsRequestSupported(old_s, domain_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMayGetVoltageLevel(old_s, calling_agent, domain_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> voltage_level == VoltageLevelDuringCommand(old_s, domain_id))
}