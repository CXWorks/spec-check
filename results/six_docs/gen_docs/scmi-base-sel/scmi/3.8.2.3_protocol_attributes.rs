pub open spec fn protocol_attributes_spec(result: int, attributes: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    && (attributes & 0xFFFF == 0)
    && (attributes & 0xFFFF == (attributes & 0xFFFF))
}