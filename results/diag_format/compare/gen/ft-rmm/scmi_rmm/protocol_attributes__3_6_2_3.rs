pub open spec fn protocol_attributes__3_6_2_3_spec(attributes: [uint32; 4], old_s: S, new_s: S) -> bool {
  (attributes[31:24] == 0)
  && (attributes[23:16] == MaxPendingAsyncClockRateChanges())
  && (attributes[15:0] == NumClocks())
  && ((attributes[23:16]) == MaxPendingAsyncClockRateChanges())
}