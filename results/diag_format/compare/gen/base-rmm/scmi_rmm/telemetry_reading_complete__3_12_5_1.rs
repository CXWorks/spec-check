pub open spec fn telemetry_reading_complete_spec(status: int, num_dwords: uint32, array: [uint32], old_s: S, new_s: S) -> bool {
    (!DeHardwareFaultDetected(old_s) ==> status != HARDWARE_ERROR)
    && (!AllTelemetryDataCollected(old_s) ==> status != PARTIAL_ERROR)
    && (status == PARTIAL_ERROR ==> !AllTelemetryDataCollected(old_s))
    && (status == HARDWARE_ERROR ==> DeHardwareFaultDetected(old_s))
    && (num_dwords % 2 == 0)
    && (AllEnabledDesOnShmtiOrFastChannel(old_s) ==> num_dwords == 0)
    && (array.len() == num_dwords)
    && (array.contains_telemetry_payload_for_enabled_des_not_on_shmti_or_fast_channel(old_s))
    && (!array.contains_prologue_or_epilogue())
}