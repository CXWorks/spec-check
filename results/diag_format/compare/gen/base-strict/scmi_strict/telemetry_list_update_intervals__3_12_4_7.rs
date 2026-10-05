pub open spec fn telemetry_list_update_intervals__3_12_4_7_spec(status: Int32, ret_flags: UInt32, intervals: Array<UInt32>, old_s: S, new_s: S) -> bool {
    (!IsValidUpdateIntervalIndex(old_s, group_identifier(old_s), flags(old_s), index(old_s)) ==> ResultEqual(status, OUT_OF_RANGE))
    && (IsValidUpdateIntervalIndex(old_s, group_identifier(old_s), flags(old_s), index(old_s)) ==> (
        ResultEqual(status, SUCCESS)
        && Bits(ret_flags, 15, 13) == 0
        && ArrayLength(intervals) == Bits(ret_flags, 11, 0)
        && (Bit(ret_flags, 12) == 1 ==> (
            Bits(ret_flags, 11, 0) == 3
            && Bits(ret_flags, 31, 16) == 0
            && ArrayEntry(intervals, 0) == LowestSupportedUpdateInterval(old_s, group_identifier(old_s), flags(old_s))
            && ArrayEntry(intervals, 1) == HighestSupportedUpdateInterval(old_s, group_identifier(old_s), flags(old_s))
            && ArrayEntry(intervals, 2) == UpdateIntervalStepSize(old_s, group_identifier(old_s), flags(old_s))
        ))
        && (Bit(ret_flags, 12) == 0 ==> (
            forall|i: UInt32| i < Bits(ret_flags, 11, 0) ==> ArrayEntry(intervals, i) == SupportedUpdateIntervalAt(old_s, group_identifier(old_s), flags(old_s), index(old_s) + i)
            && Bits(ret_flags, 31, 16) == NumSupportedUpdateIntervals(old_s, group_identifier(old_s), flags(old_s)) - index(old_s) - Bits(ret_flags, 11, 0)
        ))
        && (forall|i: UInt32| i + 1 < Bits(ret_flags, 11, 0) ==> IntervalValue(ArrayEntry(intervals, i)) < IntervalValue(ArrayEntry(intervals, i + 1)))
    ))
}