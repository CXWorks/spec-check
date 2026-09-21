pub open spec fn 3.4.2.2_negotiate_protocol_version_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> true)
    && (result == SCMI_NOT_SUPPORTED ==> true)
    && (result != SCMI_SUCCESS && result != SCMI_NOT_SUPPORTED ==> true)
}