pub open spec fn base_discover_sub_vendor_spec(result: int32, vendor_identifier: [uint8; 16], old_s: S, new_s: S) -> bool {
    (result == 0 ==> (vendor_identifier[0] == 0 && vendor_identifier[1..16] == [0u8; 15]))
    && (result != 0 ==> (vendor_identifier[0] == 0 && vendor_identifier[1..16] == [0u8; 15]))
    && (old_s == new_s)
}