pub open spec fn telemetry_update__3_12_6_1_spec(result: Int32, agent_id: UInt32, num_dwords: UInt32, array: [UInt32], old_s: S, new_s: S) -> bool {
    (!DeHardwareFaultDetected(old_s) ==> !ResultEqual(result, HARDWARE_ERROR))
    && (!SomeTelemetryDataNotCollected(old_s) ==> !ResultEqual(result, PARTIAL_ERROR))
    && (result == 0)
    && (agent_id == 0)
    && (num_dwords % 2 == 0)
    && (AllEnabledDesOnShmtiOrFastChannels(old_s) ==> num_dwords == 0)
    && (ArrayLength(array) == num_dwords)
    && (ArrayHoldsEnabledDesNotOnShmtiOrFastChannels(array))
    && (ProvidesLatestDeValues(array))
    && (AverageNotificationPeriod(old_s) >= ConfiguredSamplingPeriod(old_s))
}