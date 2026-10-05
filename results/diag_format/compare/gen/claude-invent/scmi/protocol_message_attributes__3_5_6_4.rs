pub open spec fn protocol_message_attributes__3_5_6_4_spec(message_id: u32, status: i32, attributes: u32, old_s: S, new_s: S) -> bool {
    (!IsMessageImplementedAndAvailable(old_s, message_id) ==> status == -4i32)
    && (IsMessageImplementedAndAvailable(old_s, message_id) ==> (
        status == 0i32
        && (attributes & 0xFFFF_FFFEu32) == 0u32
        && (((attributes & 1u32) == 1u32) <==> HasDedicatedFastChannel(old_s, message_id))
    ))
    && new_s == old_s
}
