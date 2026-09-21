pub open spec fn 3.3.2.2_negotiate_protocol_version_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> new_s.protocol_version == result)
    && (result == SCMI_NOT_SUPPORTED ==> new_s.protocol_version != result)
    && (result != SCMI_SUCCESS && result != SCMI_NOT_SUPPORTED ==> true)
}