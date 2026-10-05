pub open spec fn protocol_attributes__3_11_2_3_spec(result: int32, attributes_low: uint32, attributes_high: uint32, old_s: S, new_s: S) -> bool {
    (result == 0)
    && (attributes_high == 0)
    && (old_s == new_s)
}