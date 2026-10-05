pub open spec fn base_discover_sub_vendor__3_2_2_6_spec(status: int32, vendor_identifier: uint8[16], old_s: S, new_s: S) -> bool {
  ResultIsSuccess(status)
  && IsNullTerminatedAsciiString(vendor_identifier, 16)
  && (true)
}