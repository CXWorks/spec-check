pub open spec fn protocol_attributes__3_4_2_3_spec(status: i32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (attributes == 0)
    && (new_s == old_s)
}
