pub open spec fn 3.12.5.1_telemetry_reading_complete_spec(result: int, num_dwords: uint32, array: [uint32], old_s: S, new_s: S) -> bool {
    (result == SCMI_HARDWARE_ERROR ==> true)
    && (result == SCMI_PARTIAL_ERROR ==> true)
    && (result == SCMI_SUCCESS ==> (num_dwords % 2 == 0))
    && (result == SCMI_SUCCESS ==> (num_dwords == 0 || array.len() == num_dwords))
    && true
}