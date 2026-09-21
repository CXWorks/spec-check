pub open spec fn protocol_message_attributes_spec(result: int32, attributes: uint32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> attributes == 0)
    && (result == -1 ==> true)
    && (result == -2 ==> true)
    && (result == -3 ==> true)
    && (result != 0 && result != -1 && result != -2 && result != -3 ==> true)
}