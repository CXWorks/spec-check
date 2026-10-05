pub open spec fn protocol_attributes__3_6_2_3_spec(status: int32, attributes: uint32, old_s: S, new_s: S) -> bool {
  (ResultEqual(status, SUCCESS))
  && (Bits(attributes, 31, 24) == 0)
  && (Bits(attributes, 23, 16) == MaxPendingAsyncClockRateChanges())
  && (Bits(attributes, 15, 0) == NumClocks())
}