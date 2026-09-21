pub open spec fn protocol_version_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    && (result == 0x30001)
}