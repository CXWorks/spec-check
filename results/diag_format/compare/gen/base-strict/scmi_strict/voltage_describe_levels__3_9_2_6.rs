pub open spec fn voltage_describe_levels__3_9_2_6_spec(result: Int32, flags: UInt32, voltage: Array<Int32>, old_s: S, new_s: S) -> bool {
    (!VoltageDomainExists(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidVoltageLevelIndex(old_s, domain_id, level_index) ==> ResultEqual(result, OUT_OF_RANGE))
    && (!IsRequestSupported(old_s, domain_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMayGetVoltageLevels(old_s, caller, domain_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (
        Bits64(flags, 15, 13) == 0
        && (Bits64(flags, 12, 12) == 1 ==> (
            Bits64(flags, 31, 16) == 0
            && Bits64(flags, 11, 0) == 3
            && Elem(voltage, 0) == LowestVoltageLevel(old_s, domain_id)
            && Elem(voltage, 1) == HighestVoltageLevel(old_s, domain_id)
            && Elem(voltage, 2) == VoltageStepSize(old_s, domain_id)
        ))
        && (Bits64(flags, 12, 12) == 0 ==> (
            (forall|i: UInt32| i < Bits64(flags, 11, 0) ==> Elem(voltage, i) == VoltageLevelAt(old_s, domain_id, level_index + i))
            && Bits64(flags, 31, 16) == RemainingVoltageLevels(old_s, domain_id, level_index, Bits64(flags, 11, 0))
        ))
        && (forall|i: UInt32| i + 1 < Bits64(flags, 11, 0) ==> Elem(voltage, i) < Elem(voltage, i + 1))
    ))
    && (old_s == new_s)
}