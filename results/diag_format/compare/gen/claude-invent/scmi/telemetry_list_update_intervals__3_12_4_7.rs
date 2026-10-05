pub open spec fn telemetry_list_update_intervals__3_12_4_7_spec(
    index: UInt32,
    group_identifier: UInt32,
    flags: UInt32,
    status: i32,
    ret_flags: UInt32,
    intervals: Seq<UInt32>,
    old_s: S,
    new_s: S,
) -> bool {
    (!TelemetryUpdateIntervalIndexInRange(old_s, index, group_identifier, flags) ==> status == OUT_OF_RANGE)
    && (TelemetryUpdateIntervalIndexInRange(old_s, index, group_identifier, flags) ==> (
        status == SUCCESS
        && ((ret_flags >> 13u32) & 0x7u32) == 0u32
        && (((ret_flags >> 12u32) & 0x1u32) == 1u32 ==> (
            (ret_flags >> 16u32) == 0u32
            && (ret_flags & 0xFFFu32) == 3u32
        ))
        && intervals.len() == (ret_flags & 0xFFFu32) as nat
        && (((ret_flags >> 12u32) & 0x1u32) == 0u32 ==> (
            forall|i: int, j: int| 0 <= i < j < intervals.len() ==> UpdateIntervalLe(intervals[i], intervals[j])
        ))
    ))
    && new_s == old_s
}
