pub open spec fn protocol_message_attributes_spec(result: int32, attributes: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> attributes == 0)
    && (result == SCMI_NOT_FOUND ==> attributes == 0)
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND ==> attributes == 0)
    && (result == SCMI_SUCCESS ==> (attributes & 1) == 0 || (attributes & 1) == 1)
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND ==> (attributes & 1) == 0)
    && (result == SCMI_SUCCESS ==> (attributes & 0xFFFFFFFE) == 0)
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND ==> (attributes & 0xFFFFFFFE) == 0)
}