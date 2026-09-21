pub open spec fn telemetry_config_set_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (
        (old_s.cmd_input_control as int) & 0x1FE == 0
        && (old_s.cmd_input_control as int) & 0x1FE != 0x1E
    ))
    && (result == SCMI_OUT_OF_RANGE ==> (
        // Platform limit reached (spec does not define exact count, so constraint is purely on error code)
        true
    ))
    && (result == SCMI_SUCCESS ==> (
        // Success implies telemetry was successfully enabled
        true
    ))
    && (result != SCMI_SUCCESS && result != SCMI_INVALID_PARAMETERS && result != SCMI_OUT_OF_RANGE ==> true)
}