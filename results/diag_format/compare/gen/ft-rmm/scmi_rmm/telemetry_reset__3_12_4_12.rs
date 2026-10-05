pub open spec fn telemetry_reset__3_12_4_12_spec(flags: int32, status: int32, calling_agent: CallingAgent, old_s: S, new_s: S) -> bool {
  (flags != 0 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsAgentPermittedTelemetryReset(old_s, calling_agent) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> TelemetryConfigurationIsReset(new_s))
  && (ResultEqual(status, SUCCESS) ==> TelemetryAccumulatedDeDataIsReset(new_s))
  && ((!(flags != 0) &&
       IsAgentPermittedTelemetryReset(old_s, calling_agent))
    ==> ResultEqual(status, SUCCESS))
  && (ResultNotEqual(status, SUCCESS)
    ==> TelemetryConfigurationIsReset(new_s))
  && (ResultNotEqual(status, SUCCESS)
    ==> TelemetryAccumulatedDeDataIsReset(new_s))
}