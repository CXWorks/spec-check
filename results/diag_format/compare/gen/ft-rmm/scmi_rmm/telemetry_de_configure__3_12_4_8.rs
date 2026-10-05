pub open spec fn telemetry_de_configure__3_12_4_8_spec(identifier: UInt32, flags: Flags, status: Int32, SHMTI_id: UInt32, SHMTI_de_offset: UInt32, blk_ts_offset: UInt32, old_s: S, new_s: S) -> bool {
  (flags[31:4] != 0 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (flags[1:0] == 3 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (flags[2] == 1 && flags[1:0] != 0 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (flags[2] == 0 && !IsValidDeOrGroup(old_s, identifier, flags[3]) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (flags[2] == 0 && flags[3] == 1 && flags[1:0] != 0 && GroupHasEnabledDe(old_s, identifier) ==> ResultEqual(status, IN_USE))
  && (flags[2] == 0 && flags[3] == 0 && flags[1:0] == 0 && DeEnabledViaGroup(old_s, identifier) ==> ResultEqual(status, IN_USE))
  && (flags[2] == 0 && flags[1:0] != 0 && EnabledLimitReached(old_s) ==> ResultEqual(status, OUT_OF_RANGE))
  && (ResultEqual(status, SUCCESS) ==> (forall de: !DeEnabled(de) && forall g: !GroupEnabled(g)))
  && (ResultEqual(status, SUCCESS) && flags[2] == 0 && flags[3] == 0 && flags[1:0] == 0 ==> !DeEnabled(new_s, identifier))
  && (ResultEqual(status, SUCCESS) && flags[2] == 0 && flags[3] == 0 && flags[1:0] != 0 ==> DeEnabled(new_s, identifier) && DeTimestamped(new_s, identifier) == (flags[1:0] == 2))
  && (ResultEqual(status, SUCCESS) && flags[2] == 0 && flags[3] == 1 && flags[1:0] == 0 ==> forall de in GroupDes(new_s, identifier): !DeEnabled(new_s, de))
  && (ResultEqual(status, SUCCESS) && flags[2] == 0 && flags[3] == 1 && flags[1:0] != 0 ==> forall de in GroupDes(new_s, identifier): DeEnabled(new_s, de) && DeTimestamped(new_s, de) == (flags[1:0] == 2))
  && (ResultEqual(status, SUCCESS) && (!EnablingDeOrGroup(old_s, flags) || !ShmtiSupported(old_s) || !ShmtiUsedFor(old_s, identifier) || !ShmtiInfoReturnSupported(old_s)) ==> SHMTI_id == 0xFFFFFFFF)
  && (ResultEqual(status, SUCCESS) && SHMTI_id != 0xFFFFFFFF ==> SHMTI_id == ShmtiOf(new_s, identifier) && SHMTI_de_offset == DeLineMetadataOffset(new_s, identifier))
  && (ResultEqual(status, SUCCESS) && SHMTI_id != 0xFFFFFFFF && (!TimestampsEnabled(old_s, identifier) || !UsesBlockTimestamps(old_s, identifier)) ==> blk_ts_offset == 0)
  && (ResultEqual(status, SUCCESS) && SHMTI_id != 0xFFFFFFFF && TimestampsEnabled(old_s, identifier) && UsesBlockTimestamps(old_s, identifier) ==> blk_ts_offset == BlockTimestampLineOffset(new_s, identifier))
  && ((!(flags[31:4] != 0) &&
       !(flags[1:0] == 3) &&
       !(flags[2] == 1 && flags[1:0] != 0) &&
       !(flags[2] == 0 && !IsValidDeOrGroup(old_s, identifier, flags[3])) &&
       !(flags[2] == 0 && flags[3] == 1 && flags[1:0] != 0 && GroupHasEnabledDe(old_s, identifier)) &&
       !(flags[2] == 0 && flags[3] == 0 && flags[1:0] == 0 && DeEnabledViaGroup(old_s, identifier)) &&
       !(flags[2] == 0 && flags[1:0] != 0 && EnabledLimitReached(old_s)))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> SHMTI_id == SHMTI_id)
  && (result != SUCCESS
    ==> SHMTI_de_offset == SHMTI_de_offset)
  && (result != SUCCESS
    ==> blk_ts_offset == blk_ts_offset)
}