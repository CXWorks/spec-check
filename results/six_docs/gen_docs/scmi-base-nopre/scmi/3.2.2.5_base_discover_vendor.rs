pub open spec fn base_discover_vendor_spec(result: int32, vendor_identifier: [16], old_s: S, new_s: S) -> bool {
    (result == 0)
    && (vendor_identifier == [0u8; 16])
    && (old_s == new_s)
}