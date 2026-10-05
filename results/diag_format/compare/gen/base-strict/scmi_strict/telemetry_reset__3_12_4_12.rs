pub open spec fn telemetry_reset__3_12_4_12_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!AgentPermittedToResetTelemetry(calling_agent(old_s)) ==> ResultEqual(result, DENIED))
    && (flags(new_s) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (result.is_Ok() ==> (TelemetryConfigurationIsReset() && TelemetryDeDataIsCleared()))
}