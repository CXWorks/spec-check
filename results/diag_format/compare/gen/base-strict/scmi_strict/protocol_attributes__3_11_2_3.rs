pub open spec fn protocol_attributes__3_11_2_3_spec(result: int32, attributes_low: uint32, attributes_high: uint32, old_s: S, new_s: S) -> bool {
    (result == 0)
    && (Bits(attributes_low, 31, 16) == NumPinGroups())
    && (Bits(attributes_low, 15, 0) == NumPins())
    && (Bits(attributes_high, 31, 16) == 0)
    && (Bits(attributes_high, 15, 0) == NumFunctions())
    && (old_s == new_s)
}