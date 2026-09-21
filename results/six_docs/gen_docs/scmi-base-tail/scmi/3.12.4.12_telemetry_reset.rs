pub open spec fn telemetry_reset_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> <unconstrained>)
    && (result == SCMI_DENIED ==> <unconstrained>)
    && (result == SCMI_SUCCESS ==> <unconstrained>)
}