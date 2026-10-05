pub open spec fn sbi_mpxy_get_channel_ids_spec(result: sbiret, uvalue: u64, old_s: S, new_s: S) -> bool {
    (!IsValidChannelIdStartIndex(old_s.cmd_input.start_index) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (!IsMpxyShmemSetUp(old_s.calling_hart) || IsMpxyShmemDisabled(old_s.calling_hart) ==> ResultEqual(result, SBI_ERR_NO_SHMEM))
    && (!IsGetChannelIdsAllowed(old_s.calling_hart) ==> ResultEqual(result, SBI_ERR_DENIED))
    && (OtherUnspecifiedError() ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> (uvalue == 0))
    && (ResultEqual(result, SBI_SUCCESS) ==> (MpxyShmemWord(new_s.calling_hart, 0x4) == N))
    && (ResultEqual(result, SBI_SUCCESS) ==> (MpxyShmemWord(new_s.calling_hart, 0x0) == RemainingChannelIdCount(old_s.cmd_input.start_index, N)))
    && (ResultEqual(result, SBI_SUCCESS) ==> (forall i: u32 | i < N ==> MpxyShmemWord(new_s.calling_hart, 0x8 + (i as u64 * 4)) == AccessibleChannelIds()[old_s.cmd_input.start_index + i]))
}