pub open spec fn protocol_attributes__3_6_2_3_spec(result: int32, attributes: uint32, old_s: S, new_s: S) -> bool {
    (true ==> result == 0)
    && (true ==> (attributes[31..24] as uint32) == 0)
    && (true ==> (attributes[23..16] as uint32) == MaxPendingAsyncClockRateChanges())
    && (true ==> (attributes[15..0] as uint32) == NumClocks())
}