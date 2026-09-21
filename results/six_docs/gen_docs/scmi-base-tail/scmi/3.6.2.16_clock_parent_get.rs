pub open spec fn clock_parent_get_spec(result: int32, parent_id: UInt32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> parent_id == old_s.clock_parent_get_parent_id)
    && (result == SCMI_NOT_FOUND ==> true)
    && (result == SCMI_NOT_SUPPORTED ==> true)
    && (result == SCMI_DENIED ==> true)
    && (result == SCMI_GENERIC_ERROR ==> true)
    && (result == SCMI_HARDWARE_ERROR ==> true)
    && (result == SCMI_PROTOCOL_ERROR ==> true)
    && (result == SCMI_INVALID_PARAMETERS ==> true)
    && (result == SCMI_OUT_OF_RANGE ==> true)
    && (result == SCMI_PARTIAL_ERROR ==> true)
    && (result == SCMI_IN_USE ==> true)
    && (result == SCMI_BUSY ==> true)
}