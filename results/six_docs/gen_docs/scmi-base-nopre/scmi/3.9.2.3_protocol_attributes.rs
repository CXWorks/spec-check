pub open spec fn protocol_attributes_spec(result: int32, attributes: uint32, old_s: S, new_s: S) -> bool {
    (result == 0)
    && (attributes == 0)
    && (old_s == new_s)
}