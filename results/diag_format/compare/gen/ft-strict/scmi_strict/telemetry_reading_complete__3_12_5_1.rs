pub open spec fn telemetry_reading_complete__3_12_5_1_spec(status: Int32, num_dwords: UInt32, array: [UInt32; 1], old_s: S, new_s: S) -> bool {
  (DeHardwareFaultDetected(old_s) ==> ResultEqual(status, HARDWARE_ERROR))
  && (!AllTelemetryDataCollected(old_s) ==> ResultEqual(status, PARTIAL_ERROR))
  && (ResultEqual(status, SUCCESS) ==> status == SUCCESS)
  && (ResultEqual(status, SUCCESS) ==> num_dwords % 2 == 0)
  && (ResultEqual(status, SUCCESS) ==> AllEnabledDesCollectedViaShmtiOrFastChannel(old_s) ==> num_dwords == 0)
  && (ResultEqual(status, SUCCESS) ==> ArrayLength(array) == num_dwords)
  && (ResultEqual(status, SUCCESS) ==> forall|de: De| (DeEnabled(de) && DeAvailableViaShmtiOrFastChannel(de)) ==> DeDataReadableViaShmtiOrFastChannel(de))
  && (ResultEqual(status, SUCCESS) ==> forall|de: De| (DeEnabled(de) && !DeAvailableViaShmtiOrFastChannel(de)) ==> PayloadContainsLineForDe(array, de))
  && (ResultEqual(status, SUCCESS) ==> !PayloadContainsPrologue(array) && !PayloadContainsEpilogue(array))
  && ((!DeHardwareFaultDetected(old_s) &&
       AllTelemetryDataCollected(old_s))
    ==> ResultEqual(status, SUCCESS))
}