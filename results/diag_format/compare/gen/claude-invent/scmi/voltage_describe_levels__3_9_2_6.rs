pub open spec fn voltage_describe_levels__3_9_2_6_spec(domain_id: UInt32, level_index: UInt32, status: i32, flags: UInt32, voltage: Seq<i32>, old_s: S, new_s: S) -> bool {
    (!VoltageDomainExists(old_s, domain_id) ==> status == NOT_FOUND)
    && ((VoltageDomainExists(old_s, domain_id)
        && !VoltageLevelIndexInRange(old_s, domain_id, level_index)) ==> status == OUT_OF_RANGE)
    && ((VoltageDomainExists(old_s, domain_id)
        && VoltageLevelIndexInRange(old_s, domain_id, level_index)
        && !AgentAllowedToGetVoltageLevels(old_s, domain_id)) ==> status == DENIED)
    && ((VoltageDomainExists(old_s, domain_id)
        && VoltageLevelIndexInRange(old_s, domain_id, level_index)
        && AgentAllowedToGetVoltageLevels(old_s, domain_id)
        && !VoltageDescribeLevelsRequestSupported(old_s, domain_id)) ==> status == NOT_SUPPORTED)
    && ((VoltageDomainExists(old_s, domain_id)
        && VoltageLevelIndexInRange(old_s, domain_id, level_index)
        && AgentAllowedToGetVoltageLevels(old_s, domain_id)
        && VoltageDescribeLevelsRequestSupported(old_s, domain_id)) ==> status == SUCCESS)
    && (status == SUCCESS ==> (
        VoltageDomainExists(old_s, domain_id)
        && VoltageLevelIndexInRange(old_s, domain_id, level_index)
        && AgentAllowedToGetVoltageLevels(old_s, domain_id)
        && (((flags >> 13u32) & 0x7u32) == 0u32)
        && (voltage.len() == ((flags & 0xFFFu32) as int))
        && ((((flags >> 12u32) & 0x1u32) == 1u32) ==> (
            ((flags >> 16u32) as int) == 0
            && ((flags & 0xFFFu32) as int) == 3
            && (voltage[0] as int) == VoltageDomainLowestLevel(old_s, domain_id)
            && (voltage[1] as int) == VoltageDomainHighestLevel(old_s, domain_id)
            && (voltage[2] as int) == VoltageDomainStepSize(old_s, domain_id)
        ))
        && ((((flags >> 12u32) & 0x1u32) == 0u32) ==> (
            ((level_index as int) + ((flags & 0xFFFu32) as int)) <= VoltageDomainLevelCount(old_s, domain_id)
            && ((flags >> 16u32) as int) == VoltageDomainLevelCount(old_s, domain_id) - (level_index as int) - ((flags & 0xFFFu32) as int)
            && (forall|i: int| 0 <= i < voltage.len() ==>
                (voltage[i] as int) == VoltageDomainLevelAt(old_s, domain_id, (level_index as int) + i))
            && (forall|i: int, j: int| 0 <= i < j < voltage.len() ==>
                (voltage[i] as int) < (voltage[j] as int))
        ))
    ))
    && (new_s == old_s)
}
