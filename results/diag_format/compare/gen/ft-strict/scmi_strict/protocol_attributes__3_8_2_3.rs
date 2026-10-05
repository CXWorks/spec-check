pub open spec fn protocol_attributes__3_8_2_3_spec(attributes: UInt32, old_s: S, new_s: S) -> bool {
  (Bits(attributes, 31, 16) == 0)
  && (Bits(attributes, 15, 0) == NumResetDomains())
  && ((!(Bits(attributes, 31, 16) == 0))
    ==> (true))
}