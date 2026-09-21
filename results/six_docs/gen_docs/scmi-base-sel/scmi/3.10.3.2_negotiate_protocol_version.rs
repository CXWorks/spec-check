pub open spec fn 3.10.3.2_negotiate_protocol_version_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (new_s.protocol_version == result))
    && (result == SCMI_NOT_SUPPORTED ==> (new_s.protocol_version != result))
    && (result != SCMI_SUCCESS ==> (new_s.protocol_version == old_s.protocol_version))
}