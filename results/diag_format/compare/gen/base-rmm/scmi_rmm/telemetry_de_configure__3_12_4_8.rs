pub open spec fn telemetry_de_configure__3_12_4_8_spec(
    result: Int32,
    shmti_id: UInt32,
    shmti_de_offset: UInt32,
    blk_ts_offset: UInt32,
    old_s: S,
    new_s: S
) -> bool {
    let identifier: UInt32 = PayloadWord0(old_s.cmd_input);
    let flags: UInt32 = PayloadWord1(old_s.cmd_input);
    let flags_rsvd: UInt32 = flags[31..4];
    let flags_selector: UInt32 = flags[3];
    let flags_disable_all: UInt32 = flags[2];
    let flags_de_mode: UInt32 = flags[1..0];

    (flags_rsvd != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags_de_mode == 3 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags_disable_all == 1 && flags_de_mode != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags_disable_all == 0 && !IsValidDeOrGroup(identifier, flags_selector) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags_disable_all == 0 && flags_selector == 1 && flags_de_mode != 0 && GroupHasEnabledDe(identifier) ==> ResultEqual(result, IN_USE))
    && (flags_disable_all == 0 && flags_selector == 0 && flags_de_mode == 0 && DeEnabledViaGroup(identifier) ==> ResultEqual(result, IN_USE))
    && (flags_disable_all == 0 && flags_de_mode != 0 && EnabledLimitReached() ==> ResultEqual(result, OUT_OF_RANGE))
    && (ResultEqual(result, SUCCESS) ==> (
        (flags_disable_all == 1 ==> (forall de: !DeEnabled(de) && forall g: !GroupEnabled(g)))
        && (flags_disable_all == 0 && flags_selector == 0 && flags_de_mode == 0 ==> !DeEnabled(identifier))
        && (flags_disable_all == 0 && flags_selector == 0 && flags_de_mode != 0 ==> (DeEnabled(identifier) && DeTimestamped(identifier) == (flags_de_mode == 2)))
        && (flags_disable_all == 0 && flags_selector == 1 && flags_de_mode == 0 ==> forall de in GroupDes(identifier): !DeEnabled(de))
        && (flags_disable_all == 0 && flags_selector == 1 && flags_de_mode != 0 ==> (forall de in GroupDes(identifier): DeEnabled(de) && DeTimestamped(de) == (flags_de_mode == 2)))
    ))
    && (ResultEqual(result, SUCCESS) ==> (
        ((!EnablingDeOrGroup(flags) || !ShmtiSupported() || !ShmtiUsedFor(identifier) || !ShmtiInfoReturnSupported()) ==> shmti_id == 0xFFFFFFFF)
        && (shmti_id != 0xFFFFFFFF ==> (shmti_id == ShmtiOf(identifier) && shmti_de_offset == DeLineMetadataOffset(identifier)))
        && (shmti_id != 0xFFFFFFFF && (!TimestampsEnabled(identifier) || !UsesBlockTimestamps(identifier)) ==> blk_ts_offset == 0)
        && (shmti_id != 0xFFFFFFFF && TimestampsEnabled(identifier) && UsesBlockTimestamps(identifier) ==> blk_ts_offset == BlockTimestampLineOffset(identifier))
    ))
}