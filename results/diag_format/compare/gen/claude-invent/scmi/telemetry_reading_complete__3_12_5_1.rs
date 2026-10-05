pub open spec fn telemetry_reading_complete__3_12_5_1_spec(status: i32, num_dwords: u32, array: Seq<u32>, old_s: S, new_s: S) -> bool {
    (IsDeHardwareFault(old_s) ==> status == HARDWARE_ERROR)
    && ((!IsDeHardwareFault(old_s) && IsTelemetryPartiallyCollected(old_s)) ==> status == PARTIAL_ERROR)
    && ((num_dwords as int) % 2 == 0)
    && (AllEnabledDesCollectedViaShmtiOrFastChannels(old_s) ==> num_dwords == 0)
    && (array.len() == num_dwords as nat)
}
