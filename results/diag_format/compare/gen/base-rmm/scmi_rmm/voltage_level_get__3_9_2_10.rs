pub open spec fn voltage_level_get__3_9_2_10_spec(result: int32, voltage_level: int32, old_s: S, new_s: S) -> bool {
    (!IsValidVoltageDomain(old_s, domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsRequestSupported(old_s, domain_id(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMayGetVoltageLevel(old_s, caller(old_s), domain_id(old_s)) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> voltage_level == VoltageDomain(old_s, domain_id(old_s)).level)
    && (old_s == new_s)
}