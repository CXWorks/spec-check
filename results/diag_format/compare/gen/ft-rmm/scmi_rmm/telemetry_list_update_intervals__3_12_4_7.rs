pub open spec fn telemetry_list_update_intervals__3_12_4_7_spec(index: UInt32, group_identifier: UInt32, flags: Flags, status: Int32, flags1: Flags1, intervals: [UInt32; 4], old_s: S, new_s: S) -> bool {
  (!IsValidUpdateIntervalIndex(old_s, index) ==> ResultEqual(status, OUT_OF_RANGE))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> IsNumericAscending(new_s, intervals, flags1.count))
  && (ResultEqual(status, SUCCESS) ==> flags1.format == 1 ==> flags1.remaining == 0)
  && (ResultEqual(status, SUCCESS) ==> flags1.format == 1 ==> flags1.count == 3)
  && ((IsValidUpdateIntervalIndex(old_s, index))
    ==> ResultEqual(status, SUCCESS))
}