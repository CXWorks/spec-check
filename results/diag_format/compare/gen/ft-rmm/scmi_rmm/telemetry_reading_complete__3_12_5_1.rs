pub open spec fn telemetry_reading_complete__3_12_5_1_spec(message_id: UInt, protocol_id: UInt, status: int32, num_dwords: uint32, array: [uint32; 1], old_s: S, new_s: S) -> bool {
  (DeHardwareFaultDetected(old_s) ==> ResultEqual(status, HARDWARE_ERROR))
  && (!AllTelemetryDataCollected(old_s) ==> ResultEqual(status, PARTIAL_ERROR))
  && (result == RSI_SUCCESS ==> num_dwords % 2 == 0)
  && (result == RSI_SUCCESS ==> AllEnabledDesOnShmtiOrFastChannel(old_s) ==> num_dwords == 0)
  && (result == RSI_SUCCESS ==> 1 == num_dwords)
  && (result == RSI_SUCCESS ==> array contains telemetry payload lines for each DE that is enabled and not available over SHMTI or FastChannels)
  && (result == RSI_SUCCESS ==> array contains no prologue and no epilogue)
  && ((!(DeHardwareFaultDetected(old_s)) &&
       AllTelemetryDataCollected(old_s))
    ==> ResultEqual(status, RSI_SUCCESS))
}