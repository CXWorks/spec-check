pub open spec fn telemetry_list_update_intervals__3_12_4_7_spec(
    result: int32,
    old_s: S,
    new_s: S,
    index: uint32,
    group_identifier: uint32,
    flags: uint32,
    return_flags: uint32,
    num_intervals: uint32,
    intervals: [uint32],
) -> bool {
    // Failure condition: flags bits[3:0] must be 0, 1, or 2
    (flags & 0xF != 0 && flags & 0xF != 1 && flags & 0xF != 2 ==> result != 0)
    // Failure condition: if flags[3:0] == 2, group_identifier must be ignored (no specific constraint on its value, but if it's used incorrectly it might be an error; however, spec says it's ignored, so no failure condition on its value alone)
    // Failure condition: if flags[3:0] == 0, group_identifier is ignored
    // Failure condition: if flags[3:0] == 1, group_identifier must be valid (no specific constraint given, assume valid if not out of range)
    // Failure condition: index must be within valid range (no specific range given, assume valid if not out of range)
    // Failure condition: return_flags bits[15:13] must be 0
    ((return_flags >> 13) & 0x7 != 0 ==> result != 0)
    // Failure condition: if return_flags[12] == 1, then num_intervals must be 3
    (return_flags & 0x1000 != 0 && num_intervals != 3 ==> result != 0)
    // Failure condition: if return_flags[12] == 0, then num_intervals must be > 0 (implied by "number of update intervals that are returned")
    (return_flags & 0x1000 == 0 && num_intervals == 0 ==> result != 0)
    // Failure condition: if return_flags[12] == 1, then intervals[0] must be the lowest, intervals[1] must be the highest, intervals[2] must be the step size
    (return_flags & 0x1000 != 0 && (intervals[0] > intervals[1] || intervals[2] <= 0) ==> result != 0)
    // Failure condition: if return_flags[12] == 0, then intervals must be in ascending order
    (return_flags & 0x1000 == 0 && num_intervals > 1 && (intervals[0] > intervals[1] || (num_intervals > 2 && intervals[1] > intervals[2])) ==> result != 0)
    // Success condition: if all preconditions are met, result must be SUCCESS (0)
    (flags & 0xF == 0 || flags & 0xF == 1 || flags & 0xF == 2)
    && ((return_flags >> 13) & 0x7 == 0)
    && (return_flags & 0x1000 != 0 ==> num_intervals == 3)
    && (return_flags & 0x1000 == 0 ==> num_intervals > 0)
    && (return_flags & 0x1000 != 0 ==> (intervals[0] <= intervals[1] && intervals[2] > 0))
    && (return_flags & 0x1000 == 0 ==> (num_intervals > 1 ==> (intervals[0] <= intervals[1] && (num_intervals > 2 ==> intervals[1] <= intervals[2]))))
    ==> result == 0
}