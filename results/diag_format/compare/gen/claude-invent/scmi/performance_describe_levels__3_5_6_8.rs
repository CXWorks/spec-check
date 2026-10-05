pub open spec fn performance_describe_levels__3_5_6_8_spec(domain_id: UInt32, skip_index: UInt32, status: i32, num_levels: UInt32, perf_levels: Seq<(UInt32, UInt32, UInt32, UInt32, UInt32)>, old_s: S, new_s: S) -> bool {
    (!IsValidPerfDomain(old_s, domain_id) ==> status == NOT_FOUND)
    && ((IsValidPerfDomain(old_s, domain_id) && status == SUCCESS) ==> (
        perf_levels.len() == ((num_levels & 0xFFFu32) as int)
        && ((num_levels >> 12u32) & 0xFu32) == 0u32
        && (skip_index as int) + ((num_levels & 0xFFFu32) as int) + ((num_levels >> 16u32) as int) == PerfLevelCount(old_s, domain_id)
        && (forall|i: int| 0 <= i < perf_levels.len() ==> perf_levels[i] == PerfLevelEntry(old_s, domain_id, (skip_index as int) + i))
        && (forall|i: int, j: int| 0 <= i < j < perf_levels.len() ==> perf_levels[i].0 < perf_levels[j].0)
        && (forall|i: int| 0 <= i < perf_levels.len() ==> (perf_levels[i].2 >> 16u32) == 0u32)
    ))
    && new_s == old_s
}
