pub open spec fn voltage_level_set__3_9_2_9_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!VoltageDomainExists(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsSupportedVoltageLevel(old_s, domain_id, voltage_level) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsVoltageLevelSetRequestSupported(old_s, domain_id, flags, voltage_level) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMaySetVoltageLevel(old_s, calling_agent, domain_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (Bits64(flags, 0, 0) == 0 ==> VoltageLevel(new_s, domain_id) == voltage_level))
    && (ResultEqual(result, SUCCESS) ==> (Bits64(flags, 0, 0) == 1 ==> VoltageLevelSetQueued(new_s, domain_id, voltage_level)))
    && (ResultEqual(result, SUCCESS) ==> (Bits64(flags, 0, 0) == 1 ==> CompletesWithVoltageLevelSetCompleteMessage(old_s, domain_id)))
}