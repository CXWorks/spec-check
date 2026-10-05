pub open spec fn telemetry_reset_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!IsAgentPermittedTelemetryReset(calling_agent) ==> ResultEqual(result, DENIED))
    && (flags != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (result.is_Ok() ==> (TelemetryConfigurationIsReset() && TelemetryAccumulatedDeDataIsReset()))
}