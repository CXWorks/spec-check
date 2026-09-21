pub open spec fn 3.12.4.12_telemetry_reset_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> flags(old_s) != 0)
    && (result == SCMI_DENIED ==> true)
    && (result == SCMI_SUCCESS ==> flags(old_s) == 0)
    && (result == SCMI_SUCCESS ==> telemetry_reset_success(old_s, new_s))
}

pub open spec fn flags(s: S) -> int32 {
    0
}

pub open spec fn telemetry_reset_success(old_s: S, new_s: S) -> bool {
    true
}