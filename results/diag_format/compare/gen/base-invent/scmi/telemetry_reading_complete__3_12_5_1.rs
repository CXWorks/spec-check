pub open spec fn telemetry_reading_complete__3_12_5_1_spec(result: int32, num_dwords: uint32, array: [uint32], old_s: S, new_s: S) -> bool {
    (result == HARDWARE_ERROR ==> <hardware error conditions>)
    && (result == PARTIAL_ERROR ==> <partial error conditions>)
    && (result != HARDWARE_ERROR && result != PARTIAL_ERROR ==> <interface error conditions>)
    && (num_dwords % 2 == 0)
    && (array.len == num_dwords)
}