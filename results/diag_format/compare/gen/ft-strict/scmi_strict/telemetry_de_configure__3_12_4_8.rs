pub open spec fn telemetry_de_configure__3_12_4_8_spec(identifier: UInt32, flags: TelemetryDeConfigureFlags, status: Int32, SHMTI_id: UInt32, SHMTI_de_offset: UInt32, blk_ts_offset: UInt32, old_s: S, new_s: S) -> bool {
  (flags.reserved != 0 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (flags.de_mode > 2 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (flags.disable_all == 1 && flags.de_mode != 0 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (flags.disable_all == 0 && flags.selector == 0 && !IsValidDeId(identifier) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (flags.disable_all == 0 && flags.selector == 1 && !IsValidEventGroupId(identifier) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (InUseCheckImplemented() && flags.disable_all == 0 && flags.selector == 1 && flags.de_mode != 0 && (exists|de: DeId| DeInEventGroup(de, identifier) && DeIsEnabled(de)) ==> ResultEqual(status, IN_USE))
  && (InUseCheckImplemented() && flags.disable_all == 0 && flags.selector == 0 && flags.de_mode == 0 && DeEnabledByEventGroup(identifier) ==> ResultEqual(status, IN_USE))
  && (flags.disable_all == 0 && flags.de_mode != 0 && EnabledCountAtPlatformLimit() ==> ResultEqual(status, OUT_OF_RANGE))
  && (ResultEqual(status, SUCCESS) ==> status == SUCCESS)
  && (flags.disable_all == 1 ==> (forall|de: DeId| !DeIsEnabled(de)) && (forall|grp: EventGroupId| !EventGroupIsEnabled(grp)))
  && (flags.disable_all == 0 && flags.selector == 0 && flags.de_mode == 0 ==> !DeIsEnabled(identifier))
  && (flags.disable_all == 0 && flags.selector == 0 && flags.de_mode == 1 ==> DeIsEnabled(identifier) && !DeTimestampsEnabled(identifier))
  && (flags.disable_all == 0 && flags.selector == 0 && flags.de_mode == 2 ==> DeIsEnabled(identifier) && DeTimestampsEnabled(identifier))
  && (flags.disable_all == 0 && flags.selector == 1 && flags.de_mode == 0 ==> (forall|de: DeId| DeInEventGroup(de, identifier) ==> !DeIsEnabled(de)))
  && (flags.disable_all == 0 && flags.selector == 1 && flags.de_mode == 1 ==> (forall|de: DeId| DeInEventGroup(de, identifier) ==> DeIsEnabled(de) && !DeTimestampsEnabled(de)))
  && (flags.disable_all == 0 && flags.selector == 1 && flags.de_mode == 2 ==> (forall|de: DeId| DeInEventGroup(de, identifier) ==> DeIsEnabled(de) && DeTimestampsEnabled(de)))
  && (flags.disable_all == 1 || flags.de_mode == 0 ==> SHMTI_id == 0xFFFFFFFF)
  && (!ShmtiSupported() ==> SHMTI_id == 0xFFFFFFFF)
  && (!ShmtiUsedFor(identifier, flags.selector) ==> SHMTI_id == 0xFFFFFFFF)
  && (!PlatformReturnsShmtiInfo() ==> SHMTI_id == 0xFFFFFFFF)
  && (SHMTI_id != 0xFFFFFFFF ==> ShmtiAllocatedTo(SHMTI_id, identifier, flags.selector))
  && (SHMTI_id != 0xFFFFFFFF && flags.selector == 0 ==> SHMTI_de_offset == DeLineMetadataOffset(new_s, SHMTI_id, identifier))
  && (SHMTI_id != 0xFFFFFFFF && flags.selector == 1 ==> SHMTI_de_offset == MinDeLineMetadataOffsetInGroup(new_s, SHMTI_id, identifier))
  && (SHMTI_id != 0xFFFFFFFF && flags.de_mode != 2 ==> blk_ts_offset == 0)
  && (SHMTI_id != 0xFFFFFFFF && !DeUsesBlockTimestamps(identifier) ==> blk_ts_offset == 0)
  && (SHMTI_id != 0xFFFFFFFF && flags.de_mode == 2 && DeUsesBlockTimestamps(identifier) ==> blk_ts_offset == BlockTimestampLineOffset(new_s, SHMTI_id, identifier) && IsCompliantBlockTimestampLine(new_s, SHMTI_id, blk_ts_offset))
  && ((!(flags.reserved != 0) &&
       !(flags.de_mode > 2) &&
       !(flags.disable_all == 1 && flags.de_mode != 0) &&
       !(flags.disable_all == 0 && flags.selector == 0 && !IsValidDeId(identifier)) &&
       !(flags.disable_all == 0 && flags.selector == 1 && !IsValidEventGroupId(identifier)) &&
       !(InUseCheckImplemented() && flags.disable_all == 0 && flags.selector == 1 && flags.de_mode != 0 && (exists|de: DeId| DeInEventGroup(de, identifier) && DeIsEnabled(de))) &&
       !(InUseCheckImplemented() && flags.disable_all == 0 && flags.selector == 0 && flags.de_mode == 0 && DeEnabledByEventGroup(identifier)) &&
       !(flags.disable_all == 0 && flags.de_mode != 0 && EnabledCountAtPlatformLimit()))
    ==> ResultEqual(status, SUCCESS))
  && (result == RSI_SUCCESS
    ==> status == SUCCESS)
  && (result != RSI_SUCCESS
    ==> status != SUCCESS)
}