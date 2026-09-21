pub open spec fn 3.2.2.8_base_discover_list_protocols_spec(result: int, num_protocols: uint32, protocols: [uint32], old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> num_protocols == 0 && protocols == [0; 1])
    && (result == SCMI_SUCCESS ==> num_protocols > 0 && protocols == [0; 1])
    && (result != SCMI_SUCCESS && result != SCMI_SUCCESS ==> num_protocols == 0 && protocols == [0; 1])
}