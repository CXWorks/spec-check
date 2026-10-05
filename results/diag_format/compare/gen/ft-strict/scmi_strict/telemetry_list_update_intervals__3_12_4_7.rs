pub open spec fn telemetry_list_update_intervals__3_12_4_7_spec(index: UInt32, group_identifier: UInt32, flags: UInt32, status: Int32, ret_flags: UInt32, intervals: [UInt32; 4], old_s: S, new_s: S) -> bool {
  (!IsValidUpdateIntervalIndex(old_s, group_identifier, flags, index) ==> ResultEqual(status, OUT_OF_RANGE))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> Bits(ret_flags, 15, 13) == 0)
  && (ResultEqual(status, SUCCESS) ==> ArrayLength(intervals) == Bits(ret_flags, 11, 0))
  && (ResultEqual(status, SUCCESS) ==> Bit(ret_flags, 12) == 1 ==> Bits(ret_flags, 11, 0) == 3)
  && (ResultEqual(status, SUCCESS) ==> Bit(ret_flags, 12) == 1 ==> Bits(ret_flags, 31, 16) == 0)
  && (ResultEqual(status, SUCCESS) ==> Bit(ret_flags, 12) == 1 ==> ArrayEntry(intervals, 0) == LowestSupportedUpdateInterval(old_s, group_identifier, flags))
  && (ResultEqual(status, SUCCESS) ==> Bit(ret_flags, 12) == 1 ==> ArrayEntry(intervals, 1) == HighestSupportedUpdateInterval(old_s, group_identifier, flags))
  && (ResultEqual(status, SUCCESS) ==> Bit(ret_flags, 12) == 1 ==> ArrayEntry(intervals, 2) == UpdateIntervalStepSize(old_s, group_identifier, flags))
  && (ResultEqual(status, SUCCESS) ==> Bit(ret_flags, 12) == 0 ==> (forall (i: UInt32), i < Bits(ret_flags, 11, 0) ==> ArrayEntry(intervals, i) == SupportedUpdateIntervalAt(old_s, group_identifier, flags, index + i)))
  && (ResultEqual(status, SUCCESS) ==> Bit(ret_flags, 12) == 0 ==> Bits(ret_flags, 31, 16) == NumSupportedUpdateIntervals(old_s, group_identifier, flags) - index - Bits(ret_flags, 11, 0))
  && (ResultEqual(status, SUCCESS) ==> (forall (i: UInt32), i + 1 < Bits(ret_flags, 11, 0) ==> IntervalValue(ArrayEntry(intervals, i)) < IntervalValue(ArrayEntry(intervals, i + 1)))))
  && ((IsValidUpdateIntervalIndex(old_s, group_identifier, flags, index))
    ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS)
    ==> Bits(ret_flags, 15, 13) == 0)
  && (ResultEqual(status, SUCCESS)
    ==> ArrayLength(intervals) == Bits(ret_flags, 11, 0))
  && (ResultEqual(status, SUCCESS)
    ==> Bit(ret_flags, 12) == 1 ==> Bits(ret_flags, 11, 0) == 3)
  && (ResultEqual(status, SUCCESS)
    ==> Bit(ret_flags, 12) == 1 ==> Bits(ret_flags, 31, 16) == 0)
  && (ResultEqual(status, SUCCESS)
    ==> Bit(ret_flags, 12) == 1 ==> ArrayEntry(intervals, 0) == LowestSupportedUpdateInterval(old_s, group_identifier, flags))
  && (ResultEqual(status, SUCCESS)
    ==> Bit(ret_flags, 12) == 1 ==> ArrayEntry(intervals, 1) == HighestSupportedUpdateInterval(old_s, group_identifier, flags))
  && (ResultEqual(status, SUCCESS)
    ==> Bit(ret_flags, 12) == 1 ==> ArrayEntry(intervals, 2) == UpdateIntervalStepSize(old_s, group_identifier, flags))
  && (ResultEqual(status, SUCCESS)
    ==> Bit(ret_flags, 12) == 0 ==> (forall (i: UInt32), i < Bits(ret_flags, 11, 0) ==> ArrayEntry(intervals, i) == SupportedUpdateIntervalAt(old_s, group_identifier, flags, index + i)))
  && (ResultEqual(status, SUCCESS)
    ==> Bit(ret_flags, 12) == 0 ==> Bits(ret_flags, 31, 16) == NumSupportedUpdateIntervals(old_s, group_identifier, flags) - index - Bits(ret_flags, 11, 0))
  && (ResultEqual(status, SUCCESS)
    ==> (forall (i: UInt32), i + 1 < Bits(ret_flags, 11, 0) ==> IntervalValue(ArrayEntry(intervals, i)) < IntervalValue(ArrayEntry(intervals, i + 1)))))
  && (result == SUCCESS
    ==> Bits(ret_flags, 15, 13) == 0)
  && (result == SUCCESS
    ==> ArrayLength(intervals) == Bits(ret_flags, 11, 0))
  && (result == SUCCESS
    ==> Bit(ret_flags, 12) == 1 ==> Bits(ret_flags, 11, 0) == 3)
  && (result == SUCCESS
    ==> Bit(ret_flags, 12) == 1 ==> Bits(ret_flags, 31, 16) == 0)
  && (result == SUCCESS
    ==> Bit(ret_flags, 12) == 1 ==> ArrayEntry(intervals, 0) == LowestSupportedUpdateInterval(old_s, group_identifier, flags))
  && (result == SUCCESS
    ==> Bit(ret_flags, 12) == 1 ==> ArrayEntry(intervals, 1) == HighestSupportedUpdateInterval(old_s, group_identifier, flags))
  && (result == SUCCESS
    ==> Bit(ret_flags, 12) == 1 ==> ArrayEntry(intervals, 2) == UpdateIntervalStepSize(old_s, group_identifier, flags))
  && (result == SUCCESS
    ==> Bit(ret_flags, 12) == 0 ==> (forall (i: UInt32), i < Bits(ret_flags, 11, 0) ==> ArrayEntry(intervals, i) == SupportedUpdateIntervalAt(old_s, group_identifier, flags, index + i)))
  && (result == SUCCESS
    ==> Bit(ret_flags, 12) == 0 ==> Bits(ret_flags, 31, 16) == NumSupportedUpdateIntervals(old_s, group_identifier, flags) - index - Bits(ret_flags, 11, 0))
  && (result == SUCCESS
    ==> (forall (i: UInt32), i + 1 < Bits(ret_flags, 11, 0) ==> IntervalValue(ArrayEntry(intervals, i)) < IntervalValue(ArrayEntry(intervals, i + 1)))))
  && ((!(IsValidUpdateIntervalIndex(old_s, group_identifier, flags, index)))
    ==> ResultEqual(status, OUT_OF_RANGE))
  && (result != SUCCESS
    ==> Bits(ret_flags, 15, 13) == 0)
  && (result != SUCCESS
    ==> ArrayLength(intervals) == 0)
  && (result != SUCCESS
    ==> Bit(ret_flags, 12) == 0)
  && (result != SUCCESS
    ==> Bits(ret_flags, 31, 16) == 0)
  && (result != SUCCESS
    ==> ArrayEntry(intervals, 0) == 0)
  && (result != SUCCESS
    ==> ArrayEntry(intervals, 1) == 0)
  && (result != SUCCESS
    ==> ArrayEntry(intervals, 2) == 0)
  && (result != SUCCESS
    ==> (forall (i: UInt32), true))
  && (result != SUCCESS
    ==> Bits(ret_flags, 31, 16) == 0)
  && (result != SUCCESS
    ==> (forall (i: UInt32), true))
  && (result == RSI_SUCCESS
    ==> Bits(ret_flags, 15, 13) == 0)
  && (result == RSI_SUCCESS
    ==> ArrayLength(intervals) == Bits(ret_flags, 11, 0))
  && (result == RSI_SUCCESS
    ==> Bit(ret_flags, 12) == 1 ==> Bits(ret_flags, 11, 0) == 3)
  && (result == RSI_SUCCESS
    ==> Bit(ret_flags, 12) == 1 ==> Bits(ret_flags, 31, 16) == 0)
  && (result == RSI_SUCCESS
    ==> Bit(ret_flags, 12) == 1 ==> ArrayEntry(intervals, 0) == LowestSupportedUpdateInterval(old_s, group_identifier, flags))
  && (result == RSI_SUCCESS
    ==> Bit(ret_flags, 12) == 1 ==> ArrayEntry(intervals, 1) == HighestSupportedUpdateInterval(old_s, group_identifier, flags))
  && (result == RSI_SUCCESS
    ==> Bit(ret_flags, 12) == 1 ==> ArrayEntry(intervals, 2) == UpdateIntervalStepSize(old_s, group_identifier, flags))
  && (result == RSI_SUCCESS
    ==> Bit(ret_flags, 12) == 0 ==> (forall (i: UInt32), i < Bits(ret_flags, 11, 0) ==> ArrayEntry(intervals, i) == SupportedUpdateIntervalAt(old_s, group_identifier, flags, index + i)))
  && (result == RSI_SUCCESS
    ==> Bit(ret_flags, 12) == 0 ==> Bits(ret_flags, 31, 16) == NumSupportedUpdateIntervals(old_s, group_identifier, flags) - index - Bits(ret_flags, 11, 0))
  && (result == RSI_SUCCESS
    ==> (forall (i: UInt32), i + 1 < Bits(ret_flags, 11, 0) ==> IntervalValue(ArrayEntry(intervals, i)) < IntervalValue(ArrayEntry(intervals, i + 1)))))
}