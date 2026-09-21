pub open spec fn protocol_message_attributes_spec(result: int, attributes: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> attributes == 0)
    && (result == SCMI_NOT_FOUND ==> true)
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND ==> true)
}