pub open spec fn protocol_attributes_spec(result: int32, attributes_low: uint32, attributes_high: uint32, old_s: S, new_s: S) -> bool {
    (result == 0)
    && (attributes_high == 0)
    && (new_s == old_s)
}