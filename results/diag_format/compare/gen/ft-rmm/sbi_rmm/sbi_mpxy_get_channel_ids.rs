pub open spec fn sbi_mpxy_get_channel_ids_spec(start_index: UInt32, result: sbiret, uvalue: sbiret, old_s: S, new_s: S) -> bool {
  (!IsValidChannelIdStartIndex(old_s, start_index) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (!IsMpxyShmemSetUp(old_s, calling_hart(old_s)) || IsMpxyShmemDisabled(old_s, calling_hart(old_s)) ==> ResultEqual(result, SBI_ERR_NO_SHMEM))
  && (!IsGetChannelIdsAllowed(old_s, calling_hart(old_s)) ==> ResultEqual(result, SBI_ERR_DENIED))
  && (OtherUnspecifiedError(old_s) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> uvalue == 0)
  && (result == SBI_SUCCESS ==> MpxyShmemWord(new_s, calling_hart(new_s), 0x4) == MpxyShmemWord(new_s, calling_hart(new_s), 0x4))
  && (result == SBI_SUCCESS ==> MpxyShmemWord(new_s, calling_hart(new_s), 0x0) == RemainingChannelIdCount(new_s, start_index, MpxyShmemWord(new_s, calling_hart(new_s), 0x4) as int))
  && (result == SBI_SUCCESS ==> (for i in [0, MpxyShmemWord(new_s, calling_hart(new_s), 0x4) as int-1]: MpxyShmemWord(new_s, calling_hart(new_s), 0x8 + (i * 4)) == AccessibleChannelIds(new_s)[start_index + i]))
  && ((IsValidChannelIdStartIndex(old_s, start_index) &&
       IsMpxyShmemSetUp(old_s, calling_hart(old_s)) &&
       !IsMpxyShmemDisabled(old_s, calling_hart(old_s)) &&
       IsGetChannelIdsAllowed(old_s, calling_hart(old_s)) &&
       !OtherUnspecifiedError(old_s))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> MpxyShmemWord(new_s, calling_hart(new_s), 0x4) == MpxyShmemWord(old_s, calling_hart(old_s), 0x4))
  && (result != SBI_SUCCESS
    ==> MpxyShmemWord(new_s, calling_hart(new_s), 0x0) == MpxyShmemWord(old_s, calling_hart(old_s), 0x0))
  && (result != SBI_SUCCESS
    ==> MpxyShmemWord(new_s, calling_hart(new_s), 0x8 + (0 * 4)) == MpxyShmemWord(old_s, calling_hart(old_s), 0x8 + (0 * 4)))
}