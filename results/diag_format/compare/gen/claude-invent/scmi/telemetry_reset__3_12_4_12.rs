pub open spec fn telemetry_reset__3_12_4_12_spec(flags: i32, status: i32, old_s: S, new_s: S) -> bool {
    (flags != 0 ==> (status == INVALID_PARAMETERS || status == DENIED))
    && (!AgentAllowedTelemetryReset(old_s) ==> status == DENIED)
    && (status == DENIED ==> !AgentAllowedTelemetryReset(old_s))
    && ((status == INVALID_PARAMETERS || status == DENIED) ==> new_s == old_s)
    && ((TelemetryResetSupported(old_s) && flags == 0 && AgentAllowedTelemetryReset(old_s)) ==> (
        status == SUCCESS
        && TelemetryConfigurationIsReset(new_s)
        && TelemetryDeDataIsCleared(new_s)
    ))
    && (status == SUCCESS ==> (
        flags == 0
        && AgentAllowedTelemetryReset(old_s)
        && TelemetryConfigurationIsReset(new_s)
        && TelemetryDeDataIsCleared(new_s)
    ))
}
