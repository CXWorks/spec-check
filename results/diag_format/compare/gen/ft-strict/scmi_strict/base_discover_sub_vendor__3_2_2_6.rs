pub open spec fn base_discover_sub_vendor__3_2_2_6_spec(status: ResultEqual(status, SUCCESS), vendor_identifier: IsNullTerminatedAsciiString(vendor_identifier, 16), vendor_name: IsSubVendorName(vendor_identifier), old_s: S, new_s: S) -> bool {
  true
}