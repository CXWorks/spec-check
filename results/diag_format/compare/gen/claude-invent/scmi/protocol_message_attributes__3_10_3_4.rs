pub open spec fn protocol_message_attributes__3_10_3_4_spec(message_id: u32, status: i32, attributes: u32, old_s: S, new_s: S) -> bool {
    (!IsMessageImplemented(old_s, message_id) ==> status == NOT_FOUND)
    && ((IsMessageImplemented(old_s, message_id) && IsMessageAvailable(old_s, message_id)) ==> (
        status == SUCCESS
        && (attributes >> 1u32) == 0u32
        && (((attributes & 1u32) == 1u32) <==> HasDedicatedFastChannel(old_s, message_id))
    ))
    && new_s == old_s
}
