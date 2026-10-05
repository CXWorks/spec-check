pub open spec fn base_discover_sub_vendor__3_2_2_6_spec(status: Int32, vendor_identifier: UInt8[16], old_s: S, new_s: S) -> bool {
    (ResultEqual(status, SUCCESS) && IsNullTerminatedAsciiString(vendor_identifier, 16) && IsSubVendorName(vendor_identifier))
    && (old_s == new_s)
}