pub open spec fn negotiate_protocol_version_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS) || (result == SCMI_NOT_SUPPORTED)
}