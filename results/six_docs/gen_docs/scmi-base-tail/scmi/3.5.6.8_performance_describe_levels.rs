pub open spec fn 3_5_6_8_performance_describe_levels_spec(
    result: int32,
    domain_id: UInt32,
    skip_index: UInt32,
    num_levels: UInt32,
    perf_levels: &[UInt32],
    old_s: S,
    new_s: S,
) -> bool {
    (result == SCMI_SUCCESS ==> (num_levels >= 0 && perf_levels.len() as UInt32 == num_levels))
    && (result == SCMI_NOT_FOUND ==> true)
    && (result == SCMI_INVALID_PARAMETERS ==> true)
    && (result == SCMI_BUSY ==> true)
    && (result == SCMI_COMMS_ERROR ==> true)
    && (result == SCMI_DENIED ==> true)
    && (result == SCMI_HARDWARE_ERROR ==> true)
    && (result == SCMI_NOT_SUPPORTED ==> true)
    && (result == SCMI_OUT_OF_RANGE ==> true)
    && (result == SCMI_PARTIAL_ERROR ==> true)
    && (result == SCMI_PROTOCOL_ERROR ==> true)
    && (result == SCMI_IN_USE ==> true)
    && (result == SCMI_GENERIC_ERROR ==> true)
}