pub open spec fn protocol_attributes__3_4_2_3_spec(status: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
  (IsSuccessStatus(status))
  && (Bits(attributes, 31, 0) == 0)
}