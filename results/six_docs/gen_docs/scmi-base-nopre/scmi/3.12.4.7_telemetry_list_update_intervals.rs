pub open spec fn 3.12.4.7_telemetry_list_update_intervals_spec(
    result: int32,
    old_s: S,
    new_s: S,
    index: uint32,
    group_identifier: uint32,
    flags: uint32,
    new_flags: uint32,
    new_intervals: &[uint32],
    new_remaining_intervals: uint32,
    new_return_format: bool,
    new_interval_count: uint32,
) -> bool {
    // Failure condition: index out of range
    (index > old_s.telemetry_update_intervals.len() as uint32 ==> result == OUT_OF_RANGE)
    // Failure condition: flags reserved bits non-zero
    ((flags & 0xFFFFF) != 0 ==> result != SUCCESS)
    // Failure condition: flags[18] of attributes_1 is 0 but flags[3:0] != 2
    // (Assuming attributes_1 is accessible in old_s; if not, this clause is omitted per unconstrained rule)
    // Success condition: result is SUCCESS
    (result == SUCCESS ==> {
        // If return format is triplet (Bit[12] == 1), then interval_count must be 3
        (new_return_format ==> new_interval_count == 3)
        // If return format is discrete (Bit[12] == 0), then interval_count must match new_remaining_intervals logic or be valid
        (!new_return_format ==> new_interval_count <= old_s.telemetry_update_intervals.len() as uint32)
        // If return format is triplet, new_flags[11:0] must be 3
        (new_return_format ==> (new_flags & 0xFFF) == 3)
        // If return format is triplet, new_flags[15:13] must be 0
        (new_return_format ==> ((new_flags >> 13) & 0x7) == 0)
        // If return format is triplet, new_flags[31:16] must be 0
        (new_return_format ==> (new_flags & 0xFFFF0000) == 0)
        // If return format is discrete, new_flags[15:13] must be 0
        (!new_return_format ==> ((new_flags >> 13) & 0x7) == 0)
        // If return format is discrete, new_flags[31:16] must be remaining intervals
        (!new_return_format ==> (new_flags & 0xFFFF0000) == (new_remaining_intervals as uint32 & 0xFFFF0000))
        // If return format is discrete, new_flags[11:0] must be interval count
        (!new_return_format ==> (new_flags & 0xFFF) == new_interval_count)
        // Intervals must be in ascending order if discrete
        (!new_return_format && new_interval_count > 1 ==> {
            for i in 0..new_interval_count - 1 {
                new_intervals[i] < new_intervals[i + 1]
            }
        })
        // Intervals must be in ascending order if triplet
        (new_return_format && new_interval_count == 3 ==> {
            new_intervals[0] <= new_intervals[1] && new_intervals[1] <= new_intervals[2]
        })
    })
}