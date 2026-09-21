pub open spec fn base_discover_vendor_spec(result: int, vendor_identifier: [u8; 16], old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (vendor_identifier == [0u8; 16]))
    && (result != SCMI_SUCCESS ==> true)
    && (old_s == new_s)
}