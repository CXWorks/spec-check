pub open spec fn telemetry_update__3_12_6_1_spec(agent_id: UInt32, status: Int32, num_dwords: UInt32, array: [UInt32; 1], old_s: S, new_s: S) -> bool {
  (DeHardwareFaultDetected(old_s) ==> ResultEqual(status, HARDWARE_ERROR))
  && (SomeTelemetryDataNotCollected(old_s) ==> ResultEqual(status, PARTIAL_ERROR))
  && (status == 0)
  && (agent_id == 0)
  && (num_dwords % 2 == 0)
  && (AllEnabledDesOnShmtiOrFastChannels(old_s) ==> num_dwords == 0)
  && (ArrayLength(array) == num_dwords)
  && (ArrayHoldsEnabledDesNotOnShmtiOrFastChannels(array))
  && (ProvidesLatestDeValues(array))
  && (AverageNotificationPeriod(old_s) >= ConfiguredSamplingPeriod(old_s))
  && ((!DeHardwareFaultDetected(old_s) &&
       !SomeTelemetryDataNotCollected(old_s))
    ==> status == 0)
  && (result: Result<(), RmiStatusCode>,
       !result.is_Ok()
    ==> status == 0)
  && (result: Result<(), RmiStatusCode>,
       !result.is_Ok()
    ==> agent_id == 0)
  && (result: Result<(), RmiStatusCode>,
       !result.is_Ok()
    ==> num_dwords % 2 == 0)
  && (result: Result<(), RmiStatusCode>,
       !result.is_Ok()
    ==> AllEnabledDesOnShmtiOrFastChannels(old_s) ==> num_dwords == 0)
  && (result: Result<(), RmiStatusCode>,
       !result.is_Ok()
    ==> ArrayLength(array) == num_dwords)
  && (result: Result<(), RmiStatusCode>,
       !result.is_Ok()
    ==> ArrayHoldsEnabledDesNotOnShmtiOrFastChannels(array))
  && (result: Result<(), RmiStatusCode>,
       !result.is_Ok()
    ==> ProvidesLatestDeValues(array))
  && (result: Result<(), RmiStatusCode>,
       !result.is_Ok()
    ==> AverageNotificationPeriod(old_s) >= ConfiguredSamplingPeriod(old_s))
}