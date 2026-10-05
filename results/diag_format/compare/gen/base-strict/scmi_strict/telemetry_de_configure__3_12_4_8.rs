pub open spec fn telemetry_de_configure__3_12_4_8_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (flags_reserved_pre(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (de_mode_reserved_pre(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (disable_all_mode_pre(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (de_identifier_pre(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (group_identifier_pre(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (group_de_in_use_pre(old_s) ==> ResultEqual(result, IN_USE))
    && (de_group_in_use_pre(old_s) ==> ResultEqual(result, IN_USE))
    && (enable_limit_pre(old_s) ==> ResultEqual(result, OUT_OF_RANGE))
    && (ResultEqual(result, SUCCESS) ==> (
        (flags.disable_all == 1 ==> (forall|de: DeId| !DeIsEnabled(de)) && (forall|grp: EventGroupId| !EventGroupIsEnabled(grp)))
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
        && (SHMTI_id != 0xFFFFFFFF && flags.selector == 0 ==> SHMTI_de_offset == DeLineMetadataOffset(SHMTI_id, identifier))
        && (SHMTI_id != 0xFFFFFFFF && flags.selector == 1 ==> SHMTI_de_offset == MinDeLineMetadataOffsetInGroup(SHMTI_id, identifier))
        && (SHMTI_id != 0xFFFFFFFF && flags.de_mode != 2 ==> blk_ts_offset == 0)
        && (SHMTI_id != 0xFFFFFFFF && !DeUsesBlockTimestamps(identifier) ==> blk_ts_offset == 0)
        && (SHMTI_id != 0xFFFFFFFF && flags.de_mode == 2 && DeUsesBlockTimestamps(identifier) ==> blk_ts_offset == BlockTimestampLineOffset(SHMTI_id, identifier) && IsCompliantBlockTimestampLine(SHMTI_id, blk_ts_offset))
    ))
}

fn flags_reserved_pre(s: S) -> bool {
    flags.reserved != 0
}

fn de_mode_reserved_pre(s: S) -> bool {
    flags.de_mode > 2
}

fn disable_all_mode_pre(s: S) -> bool {
    flags.disable_all == 1 && flags.de_mode != 0
}

fn de_identifier_pre(s: S) -> bool {
    flags.disable_all == 0 && flags.selector == 0 && !IsValidDeId(identifier)
}

fn group_identifier_pre(s: S) -> bool {
    flags.disable_all == 0 && flags.selector == 1 && !IsValidEventGroupId(identifier)
}

fn group_de_in_use_pre(s: S) -> bool {
    InUseCheckImplemented() && flags.disable_all == 0 && flags.selector == 1 && flags.de_mode != 0 && (exists|de: DeId| DeInEventGroup(de, identifier) && DeIsEnabled(de))
}

fn de_group_in_use_pre(s: S) -> bool {
    InUseCheckImplemented() && flags.disable_all == 0 && flags.selector == 0 && flags.de_mode == 0 && DeEnabledByEventGroup(identifier)
}

fn enable_limit_pre(s: S) -> bool {
    flags.disable_all == 0 && flags.de_mode != 0 && EnabledCountAtPlatformLimit()
}