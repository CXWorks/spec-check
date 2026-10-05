pub open spec fn telemetry_de_configure__3_12_4_8_spec(identifier: u32, flags: u32, status: i32, shmti_id: u32, shmti_de_offset: u32, blk_ts_offset: u32, old_s: S, new_s: S) -> bool {
    ((((flags >> 4u32) != 0u32) || ((flags & 0x3u32) == 3u32) || ((((flags >> 2u32) & 1u32) == 1u32) && ((flags & 0x3u32) != 0u32)) || ((((flags >> 2u32) & 1u32) == 0u32) && (((flags >> 3u32) & 1u32) == 0u32) && !TelemetryDeIdValid(old_s, identifier)) || ((((flags >> 2u32) & 1u32) == 0u32) && (((flags >> 3u32) & 1u32) == 1u32) && !TelemetryEventGroupIdValid(old_s, identifier))) ==> (status == INVALID_PARAMETERS))
    && ((((flags >> 4u32) == 0u32) && ((flags & 0x3u32) != 0u32) && ((flags & 0x3u32) != 3u32) && (((flags >> 2u32) & 1u32) == 0u32) && ((((flags >> 3u32) & 1u32) == 0u32 && TelemetryDeIdValid(old_s, identifier)) || (((flags >> 3u32) & 1u32) == 1u32 && TelemetryEventGroupIdValid(old_s, identifier))) && TelemetryEnableLimitReached(old_s, identifier, ((flags >> 3u32) & 1u32) == 1u32)) ==> (status == OUT_OF_RANGE))
    && ((status == IN_USE) ==> TelemetryConfigInUse(old_s, identifier, flags))
    && ((status != SUCCESS) ==> (new_s == old_s && shmti_id == 0xFFFF_FFFFu32))
    && ((((flags >> 4u32) == 0u32) && (((((flags >> 2u32) & 1u32) == 1u32) && ((flags & 0x3u32) == 0u32)) || ((((flags >> 2u32) & 1u32) == 0u32) && ((flags & 0x3u32) != 3u32) && ((((flags >> 3u32) & 1u32) == 0u32 && TelemetryDeIdValid(old_s, identifier)) || (((flags >> 3u32) & 1u32) == 1u32 && TelemetryEventGroupIdValid(old_s, identifier))) && (((flags & 0x3u32) == 0u32) || !TelemetryEnableLimitReached(old_s, identifier, ((flags >> 3u32) & 1u32) == 1u32)))) && !TelemetryConfigInUse(old_s, identifier, flags)) ==> (status == SUCCESS))
    && ((status == SUCCESS) ==> (
        ((((flags >> 2u32) & 1u32) == 1u32) ==> (TelemetryAllDisabled(new_s) && shmti_id == 0xFFFF_FFFFu32 && blk_ts_offset == 0u32))
        && ((((flags >> 2u32) & 1u32) == 0u32 && ((flags >> 3u32) & 1u32) == 0u32) ==> (
            TelemetryConfigUnchangedExcept(old_s, new_s, identifier, false)
            && (((flags & 0x3u32) == 0u32) ==> (!TelemetryDeEnabled(new_s, identifier) && shmti_id == 0xFFFF_FFFFu32))
            && (((flags & 0x3u32) == 1u32) ==> (TelemetryDeEnabled(new_s, identifier) && !TelemetryDeTimestampEnabled(new_s, identifier)))
            && (((flags & 0x3u32) == 2u32) ==> (TelemetryDeEnabled(new_s, identifier) && TelemetryDeTimestampEnabled(new_s, identifier)))))
        && ((((flags >> 2u32) & 1u32) == 0u32 && ((flags >> 3u32) & 1u32) == 1u32) ==> (
            TelemetryConfigUnchangedExcept(old_s, new_s, identifier, true)
            && (((flags & 0x3u32) == 0u32) ==> (TelemetryGroupAllDisabled(new_s, identifier) && shmti_id == 0xFFFF_FFFFu32))
            && (((flags & 0x3u32) == 1u32) ==> (TelemetryGroupAllEnabled(new_s, identifier) && !TelemetryGroupTimestampEnabled(new_s, identifier)))
            && (((flags & 0x3u32) == 2u32) ==> (TelemetryGroupAllEnabled(new_s, identifier) && TelemetryGroupTimestampEnabled(new_s, identifier)))))
        && (((flags & 0x3u32) != 2u32) ==> (blk_ts_offset == 0u32))
    ))
}
