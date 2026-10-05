pub open spec fn telemetry_de_enabled_list__3_12_4_9_spec(index: UInt32, flags: UInt32, status: Int32, ret_flags: UInt32, array: [UInt32; 1], result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!IsValidEnabledListIndex(old_s, index, Bits(flags, 0, 0)) ==> result == OUT_OF_RANGE)
  && (result == SUCCESS ==> status == SUCCESS)
  && (result == SUCCESS ==> Bits(ret_flags, 15, 0) == ArrayLength(new_s, array))
  && (result == SUCCESS ==> Bits(ret_flags, 31, 16) == RemainingEnabledElements(new_s, index, Bits(flags, 0, 0), Bits(ret_flags, 15, 0)))
  && (result == SUCCESS ==> forall i: UInt32, i < Bits(ret_flags, 15, 0) ==> IsEnabledElement(new_s, ArrayEntryWord(new_s, array, i, 0), Bits(flags, 0, 0)))
  && (result == SUCCESS ==> forall i: UInt32, i < Bits(ret_flags, 15, 0) ==> Bits(ArrayEntryWord(new_s, array, i, 1), 31, 2) == 0)
  && (result == SUCCESS ==> forall i: UInt32, i < Bits(ret_flags, 15, 0) ==> (Bits(ArrayEntryWord(new_s, array, i, 1), 1, 0) == 1 || Bits(ArrayEntryWord(new_s, array, i, 1), 1, 0) == 2))
  && (result == SUCCESS ==> forall i: UInt32, i < Bits(ret_flags, 15, 0) ==> (Bits(ArrayEntryWord(new_s, array, i, 1), 1, 0) == 2) == IsEnabledWithTimestamps(new_s, ArrayEntryWord(new_s, array, i, 0), Bits(flags, 0, 0)))
  && ((IsValidEnabledListIndex(old_s, index, Bits(flags, 0, 0)))
    ==> result == SUCCESS)
}