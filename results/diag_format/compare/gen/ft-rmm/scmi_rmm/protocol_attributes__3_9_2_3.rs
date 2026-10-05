pub open spec fn protocol_attributes__3_9_2_3_spec(attributes: UInt32, old_s: S, new_s: S) -> bool {
  (attributes[15:0] == NumVoltageDomains())
}