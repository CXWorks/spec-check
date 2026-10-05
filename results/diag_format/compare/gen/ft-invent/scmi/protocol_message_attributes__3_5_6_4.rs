pub open spec fn protocol_message_attributes__3_5_6_4_spec(message_id: UInt32, result: Result<int32, UInt32>, attributes: UInt32, old_s: S, new_s: S) -> bool {
  (result == SUCCESS ==> (attributes & 1) == 0)
}