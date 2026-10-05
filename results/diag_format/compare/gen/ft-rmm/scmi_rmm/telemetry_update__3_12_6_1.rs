pub open spec fn telemetry_update__3_12_6_1_spec(agent_id: UInt32, status: Int32, num_dwords: UInt32, array: [UInt32; 1], old_s: S, new_s: S) -> bool {
  (SensorHasFault(old_s, DE) ==> ResultEqual(status, HARDWARE_ERROR))
  && (!AllTelemetryDataCollected(old_s) ==> ResultEqual(status, PARTIAL_ERROR))
  && (status == 0)
  && (agent_id == 0)
  && ((num_dwords % 2) == 0)
  && (AllEnabledDesCollectedViaShmtiOrFastChannel(old_s) ==> num_dwords == 0)
  && (1 == num_dwords)
  && (PayloadContainsLatestDeValues(array))
  && ((!(SensorHasFault(old_s, DE)) &&
       AllTelemetryDataCollected(old_s))
    ==> ResultEqual(status, 0))
}