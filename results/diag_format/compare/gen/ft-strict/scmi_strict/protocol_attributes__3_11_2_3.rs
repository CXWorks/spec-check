pub open spec fn protocol_attributes__3_11_2_3_spec(attributes_low: uint32, attributes_high: uint32, result: int32, old_s: S, new_s: S) -> bool {
  (Bits(attributes_low, 31, 16) == NumPinGroups())
  && (Bits(attributes_low, 15, 0) == NumPins())
  && (Bits(attributes_high, 31, 16) == 0)
  && (Bits(attributes_high, 15, 0) == NumFunctions())
  && ((!(result == 0)) ==> (Bits(attributes_low, 31, 16) == NumPinGroups()))
  && ((!(result == 0)) ==> (Bits(attributes_low, 15, 0) == NumPins()))
  && ((!(result == 0)) ==> (Bits(attributes_high, 31, 16) == 0))
  && ((!(result == 0)) ==> (Bits(attributes_high, 15, 0) == NumFunctions()))
}