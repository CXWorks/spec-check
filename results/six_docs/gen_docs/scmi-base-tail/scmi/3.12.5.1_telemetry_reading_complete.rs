pub open spec fn telemetry_reading_complete_spec(result: int32, num_dwords: uint32, array: array<uint32>, old_s: S, new_s: S) -> bool {
    (result == SCMI_HARDWARE_ERROR ==> <true>)
    && (result == SCMI_PARTIAL_ERROR ==> <true>)
    && (result == SCMI_SUCCESS ==> (num_dwords % 2 == 0))
    && <true>
}