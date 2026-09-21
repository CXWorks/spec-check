pub open spec fn protocol_attributes_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    && (old_s == new_s)
}