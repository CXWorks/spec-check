pub open spec fn protocol_message_attributes__3_2_2_4_spec(message_id: UInt32, status: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
  (!IsMessageImplemented(old_s, 0x10, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> IsMessageImplemented(new_s, 0x10, message_id) && IsMessageAvailable(new_s, 0x10, message_id))
  && (ResultEqual(status, SUCCESS) ==> attributes == 0)
  && ((!(IsMessageImplemented(old_s, 0x10, message_id)))
    ==> ResultEqual(status, SUCCESS))
}