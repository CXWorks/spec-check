pub open spec fn telemetry_config_set__3_12_4_10_spec(
    group_identifier: UInt32,
    control: UInt32,
    sampling_rate: UInt32,
    status: i32,
    old_s: S,
    new_s: S,
) -> bool {
    (((control >> 9u32) != 0u32) ==> status == INVALID_PARAMETERS)
    && ((((control >> 5u32) & 0xFu32) > 2u32) ==> status == INVALID_PARAMETERS)
    && (((((TelemetryProtocolAttributes1(old_s) >> 18u32) & 1u32) == 0u32)
        && (((control >> 5u32) & 0xFu32) != 2u32)) ==> status == INVALID_PARAMETERS)
    && (((((control >> 5u32) & 0xFu32) == 1u32)
        && !IsValidEventGroup(old_s, group_identifier)) ==> status == INVALID_PARAMETERS)
    && ((((control & 1u32) == 1u32)
        && (((control >> 1u32) & 0xFu32) > 2u32)) ==> status == INVALID_PARAMETERS)
    && ((((control & 1u32) == 1u32)
        && !TelemetryModeSupported(old_s, (control >> 1u32) & 0xFu32)) ==> status == INVALID_PARAMETERS)
    && ((((control & 1u32) == 1u32)
        && !AnyDeEnabled(old_s, (control >> 5u32) & 0xFu32, group_identifier)) ==> status == INVALID_PARAMETERS)
    && ((((control & 1u32) == 1u32)
        && TelemetryEnableLimitReached(old_s, (control >> 5u32) & 0xFu32, group_identifier, (control >> 1u32) & 0xFu32)) ==> status == OUT_OF_RANGE)
    && ((status != SUCCESS) ==> new_s == old_s)
    && (((((control >> 9u32) == 0u32)
        && (((control >> 5u32) & 0xFu32) <= 2u32)
        && !((((TelemetryProtocolAttributes1(old_s) >> 18u32) & 1u32) == 0u32)
             && (((control >> 5u32) & 0xFu32) != 2u32))
        && !((((control >> 5u32) & 0xFu32) == 1u32) && !IsValidEventGroup(old_s, group_identifier))
        && ((control & 1u32) == 0u32)))
        ==> (status == SUCCESS
            && !TelemetryCollectionEnabled(new_s, (control >> 5u32) & 0xFu32, group_identifier)))
    && (((((control >> 9u32) == 0u32)
        && (((control >> 5u32) & 0xFu32) <= 2u32)
        && !((((TelemetryProtocolAttributes1(old_s) >> 18u32) & 1u32) == 0u32)
             && (((control >> 5u32) & 0xFu32) != 2u32))
        && !((((control >> 5u32) & 0xFu32) == 1u32) && !IsValidEventGroup(old_s, group_identifier))
        && ((control & 1u32) == 1u32)
        && (((control >> 1u32) & 0xFu32) <= 2u32)
        && TelemetryModeSupported(old_s, (control >> 1u32) & 0xFu32)
        && AnyDeEnabled(old_s, (control >> 5u32) & 0xFu32, group_identifier)
        && !TelemetryEnableLimitReached(old_s, (control >> 5u32) & 0xFu32, group_identifier, (control >> 1u32) & 0xFu32)))
        ==> (status == SUCCESS
            && TelemetryCollectionEnabled(new_s, (control >> 5u32) & 0xFu32, group_identifier)
            && TelemetryModeApplied(old_s, new_s, (control >> 1u32) & 0xFu32)
            && ((((control >> 1u32) & 0xFu32) != 2u32)
                ==> TelemetrySamplingRateApplied(new_s, (control >> 5u32) & 0xFu32, group_identifier, sampling_rate & 0x1F_FFFFu32))))
}
