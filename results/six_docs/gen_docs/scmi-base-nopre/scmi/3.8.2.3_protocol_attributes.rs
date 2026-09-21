pub open spec fn protocol_attributes_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0)
    && (new_s.impl_attributes == old_s.impl_attributes)
}