pub open spec fn 3.12.4.7_telemetry_list_update_intervals_spec(result: int, flags: uint32, intervals: array<uint32>, index: uint32, group_identifier: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (
        (flags & 0xFFFF == 0)
        && (index < 0xFFFFFFFF)
        && (group_identifier < 0xFFFFFFFF)
        && (intervals.len() == (flags & 0xFFFF) as int)
        && (forall i: int | 0 <= i && i < intervals.len() ==> (
            (intervals[i] as int) >= 0
        ))
    ))
    && (result != SCMI_SUCCESS ==> (
        (result == SCMI_OUT_OF_RANGE ==> (index >= 0xFFFFFFFF))
        && (result == SCMI_INVALID_PARAMETERS ==> (
            (flags & 0xFFFF != 0)
            || (index >= 0xFFFFFFFF)
            || (group_identifier >= 0xFFFFFFFF)
        ))
    ))
}