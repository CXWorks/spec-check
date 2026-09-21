pub open spec fn protocol_message_attributes_spec(result: (int32, uint32), old_s: S, new_s: S) -> bool {
    let (status, attributes) = result;
    (status == 0 ==> attributes == 0)
    && (status == 0 ==> (attributes & 0x1) == 0 || (attributes & 0x1) == 1)
    && (status != 0 ==> attributes == 0)
    && (status == 0 ==> (attributes & 0xFFFFFFFE) == 0)
}