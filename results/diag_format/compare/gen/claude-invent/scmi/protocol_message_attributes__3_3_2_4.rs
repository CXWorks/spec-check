pub open spec fn protocol_message_attributes__3_3_2_4_spec(status: i32, attributes: u32, message_id: u32, old_s: S, new_s: S) -> bool {
    (!PowerDomainMessageImplementedAndAvailable(old_s, message_id) ==> status == NOT_FOUND)
    && (PowerDomainMessageImplementedAndAvailable(old_s, message_id) ==> (status == SUCCESS && attributes == 0))
    && (new_s == old_s)
}
