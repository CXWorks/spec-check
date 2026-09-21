pub open spec fn protocol_message_attributes_spec(result: int32, attributes: uint32, old_s: S, new_s: S) -> bool {
    (result == 0x16_0000_0000_0000_0000 as int32 ==> attributes == 0)
    && (result == 0x16_0000_0000_0000_0000 as int32 ==> old_s.protocol_message_attributes == attributes)
    && (result != 0x16_0000_0000_0000_0000 as int32 ==> attributes == 0)
}