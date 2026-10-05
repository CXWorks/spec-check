pub open spec fn telemetry_update__3_12_6_1_spec(status: Int32, agent_id: UInt32, num_dwords: UInt32, array: [UInt32], old_s: S, new_s: S) -> bool {
    (!SensorHasFault(old_s) ==> status != HARDWARE_ERROR)
    && (AllTelemetryDataCollected(old_s) ==> status != PARTIAL_ERROR)
    && (status == 0)
    && (agent_id == 0)
    && ((num_dwords % 2) == 0)
    && (AllEnabledDesCollectedViaShmtiOrFastChannel(old_s) ==> num_dwords == 0)
    && (array.len() == num_dwords)
    && (PayloadContainsLatestDeValues(array))
}