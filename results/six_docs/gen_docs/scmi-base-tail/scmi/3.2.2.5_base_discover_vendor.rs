pub open spec fn 3.2.2.5_base_discover_vendor_spec(result: int32, vendor_identifier: [uint8; 16], old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (vendor_identifier == [0u8; 16]))
    && (result != SCMI_SUCCESS ==> true)
}