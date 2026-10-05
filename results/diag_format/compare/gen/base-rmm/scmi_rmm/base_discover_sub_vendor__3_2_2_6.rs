pub open spec fn base_discover_sub_vendor__3_2_2_6_spec(status: int, vendor_identifier: [16]uint8, old_s: S, new_s: S) -> bool {
    (ResultIsSuccess(status) ==> IsNullTerminatedAsciiString(vendor_identifier, 16))
    && (ResultIsSuccess(status) ==> new_s == old_s)
}