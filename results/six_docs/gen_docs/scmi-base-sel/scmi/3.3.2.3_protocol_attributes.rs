pub open spec fn protocol_attributes_spec(result: int, attributes: uint32, statistics_address_low: uint32, statistics_address_high: uint32, statistics_len: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (attributes == 0 && statistics_address_low == 0 && statistics_address_high == 0 && statistics_len == 0))
    && (result != SCMI_SUCCESS ==> (attributes == 0 && statistics_address_low == 0 && statistics_address_high == 0 && statistics_len == 0))
}