pub open spec fn protocol_message_attributes__3_2_2_4_spec(message_id: uint32, status: int32, attributes: uint32, old_s: S, new_s: S) -> bool {
  (!IsMessageImplemented(old_s, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (result == SUCCESS ==> ResultEqual(status, SUCCESS))
  && (result == SUCCESS ==> attributes == 0)
  && ((!(IsMessageImplemented(old_s, message_id)))
    ==> ResultEqual(status, SUCCESS))
}