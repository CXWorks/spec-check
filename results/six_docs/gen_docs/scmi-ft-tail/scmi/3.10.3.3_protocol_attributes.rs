pub open spec fn 3.10.3.3_protocol_attributes_spec(attributes: UInt32, old_s: S, new_s: S) -> bool {
  (attributes & 0xFFFF == 0)
}