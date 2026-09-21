pub open spec fn protocol_attributes_spec(result: int32, attributes: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    && (attributes == 0)
}