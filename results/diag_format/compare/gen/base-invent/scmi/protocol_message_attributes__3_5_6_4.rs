pub open spec fn protocol_message_attributes__3_5_6_4_spec(result: int32, attributes: uint32, old_s: S, new_s: S) -> bool {
    (result == 0) ==> (attributes == 0 || (attributes & 1) == 1)
}