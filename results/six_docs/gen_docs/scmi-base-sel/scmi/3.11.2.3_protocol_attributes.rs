pub open spec fn protocol_attributes_spec(result: int, attributes_low: uint32, attributes_high: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    && (attributes_high == 0)
    && (old_s == new_s)
}