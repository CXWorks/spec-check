pub open spec fn protocol_attributes__3_10_3_3_spec(attributes: uint32, old_s: S, new_s: S) -> bool {
  (attributes[31:16] == 0)
  && (attributes[15:0] == NumPowerCappingDomains())
}