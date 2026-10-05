pub open spec fn voltage_describe_levels__3_9_2_6_spec(status: int32, flags: uint32, voltage: int32, num_voltage: uint32, domain_id: uint32, level_index: uint32, old_s: S, new_s: S) -> bool {
    (!VoltageDomainExists(domain_id) ==> ResultEqual(status, NOT_FOUND))
    && (!IsValidVoltageLevelIndex(domain_id, level_index) ==> ResultEqual(status, OUT_OF_RANGE))
    && (!IsRequestSupported(VOLTAGE_DESCRIBE_LEVELS) ==> ResultEqual(status, NOT_SUPPORTED))
    && (!AgentMayGetVoltageLevels(calling_agent, domain_id) ==> ResultEqual(status, DENIED))
    && (ResultEqual(status, SUCCESS) ==> (flags[15:13] == 0))
    && (ResultEqual(status, SUCCESS) ==> (num_voltage == flags[11:0]))
    && (flags[12] == 1 ==> (flags[11:0] == 3 && flags[15:13] == 0 && voltage[0] == LowestVoltageLevel(domain_id) && voltage[1] == HighestVoltageLevel(domain_id) && voltage[2] == VoltageStepSize(domain_id)))
    && (flags[12] == 0 ==> (forall i: int | i < num_voltage ==> IsSupportedVoltageLevel(domain_id, voltage[i])))
    && (forall i: int | i < num_voltage - 1 ==> voltage[i] <= voltage[i + 1])
}