pub open spec fn sbi_mpxy_get_channel_ids_spec(result: SbiErrorCode, uvalue: UInt, old_s: S, new_s: S) -> bool {
    (!IsValidChannelIdStartIndex(old_s, start_index) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (!MpxyShmemEnabled(old_s, CallingHart()) ==> ResultEqual(result, SBI_ERR_NO_SHMEM))
    && (!GetChannelIdsAllowed(old_s, CallingHart()) ==> ResultEqual(result, SBI_ERR_DENIED))
    && (UnspecifiedFailure(old_s, CallingHart()) ==> ResultEqual(result, SBI_ERR_FAILED))
    && (result == SBI_SUCCESS ==> uvalue == 0)
    && (result == SBI_SUCCESS ==> ShmemReturned(new_s, CallingHart()) == NumChannelIdsWritten(new_s, CallingHart()))
    && (result == SBI_SUCCESS ==> RemainingCountsChannelIdsAfterReturned(new_s, CallingHart(), start_index, ShmemRemaining(new_s, CallingHart())))
    && (result == SBI_SUCCESS ==> forall|i: UInt32| i < ShmemReturned(new_s, CallingHart()) ==> ShmemChannelId(new_s, CallingHart(), i) == AccessibleChannelId(new_s, CallingHart(), start_index + i))
}