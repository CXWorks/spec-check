pub open spec fn protocol_attributes_spec(result: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> attributes == 0)
    && (result != SCMI_SUCCESS ==> attributes == 0)
    && (old_s == new_s)
}