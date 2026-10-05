pub open spec fn protocol_message_attributes__3_8_2_4_spec(message_id: UInt32, status: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
  (!IsMessageImplemented(old_s, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (result == SUCCESS ==> status == SUCCESS)
  && (result == SUCCESS ==> attributes == 0)
  && ((!(IsMessageImplemented(old_s, message_id)))
    ==> ResultEqual(status, SUCCESS))
}