pub open spec fn telemetry_reset__3_12_4_12_spec(flags: Int32, status: Int32, old_s: S, new_s: S) -> bool {
  (flags != 0 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!AgentPermittedToResetTelemetry(old_s, calling_agent) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> TelemetryConfigurationIsReset())
  && (ResultEqual(status, SUCCESS) ==> TelemetryDeDataIsCleared())
  && ((!(flags != 0) &&
       AgentPermittedToResetTelemetry(old_s, calling_agent))
    ==> ResultEqual(status, SUCCESS))
}