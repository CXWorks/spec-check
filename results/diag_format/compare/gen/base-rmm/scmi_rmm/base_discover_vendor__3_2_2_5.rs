pub open spec fn base_discover_vendor__3_2_2_5_spec(result: int32, vendor_identifier: uint8[16], old_s: S, new_s: S) -> bool {
    (true ==> result == 0)
    && (true ==> IsAsciiString(vendor_identifier))
    && (true ==> old_s == new_s)
}