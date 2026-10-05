pub open spec fn sbi_mpxy_get_channel_ids_spec(start_index: UInt32, result: SbiErrorCode, uvalue: UInt, old_s: S, new_s: S) -> bool {
  (!IsValidChannelIdStartIndex(old_s, start_index) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (!MpxyShmemEnabled(old_s, CallingHart(old_s)) ==> ResultEqual(result, SBI_ERR_NO_SHMEM))
  && (!GetChannelIdsAllowed(old_s, CallingHart(old_s)) ==> ResultEqual(result, SBI_ERR_DENIED))
  && (UnspecifiedFailure(old_s, CallingHart(old_s)) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> uvalue == 0)
  && (result == SBI_SUCCESS ==> ShmemReturned(new_s, CallingHart(new_s)) == NumChannelIdsWritten(new_s, CallingHart(new_s)))
  && (result == SBI_SUCCESS ==> RemainingCountsChannelIdsAfterReturned(new_s, CallingHart(new_s), start_index, ShmemRemaining(new_s, CallingHart(new_s))))
  && (result == SBI_SUCCESS ==> (forall (i: UInt32), i < ShmemReturned(new_s, CallingHart(new_s)) ==> ShmemChannelId(new_s, CallingHart(new_s), i) == AccessibleChannelId(new_s, CallingHart(new_s), start_index + i)))
  && ((IsValidChannelIdStartIndex(old_s, start_index) &&
       MpxyShmemEnabled(old_s, CallingHart(old_s)) &&
       GetChannelIdsAllowed(old_s, CallingHart(old_s)) &&
       !UnspecifiedFailure(old_s, CallingHart(old_s)))
    ==> ResultEqual(result, SBI_SUCCESS))
  && (result != SBI_SUCCESS
    ==> ShmemReturned(new_s, CallingHart(new_s)) == ShmemReturned(old_s, CallingHart(old_s)))
  && (result != SBI_SUCCESS
    ==> ShmemRemaining(new_s, CallingHart(new_s)) == ShmemRemaining(old_s, CallingHart(old_s)))
  && (result != SBI_SUCCESS
    ==> ShmemReturned(new_s, CallingHart(new_s)) == 0)
  && (result != SBI_SUCCESS
    ==> ShmemChannelIdArray(new_s, CallingHart(new_s)) == ShmemChannelIdArray(old_s, CallingHart(old_s)))
}