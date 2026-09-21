pub open spec fn base_discover_sub_vendor_spec(result: int32, vendor_identifier: uint8[16], old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> vendor_identifier[0] == 0)
    && (result == SCMI_SUCCESS ==> vendor_identifier[1..16] == vendor_identifier[1..16])
    && (result != SCMI_SUCCESS ==> vendor_identifier[0] == 0)
    && (result != SCMI_SUCCESS ==> vendor_identifier[1..16] == vendor_identifier[1..16])
    && (result == SCMI_SUCCESS ==> true)
    && (result != SCMI_SUCCESS ==> true)
}