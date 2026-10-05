pub open spec fn base_discover_vendor__3_2_2_5_spec(vendor_identifier: [uint8; 16], old_s: S, new_s: S) -> bool {
  IsAsciiString(vendor_identifier)
}