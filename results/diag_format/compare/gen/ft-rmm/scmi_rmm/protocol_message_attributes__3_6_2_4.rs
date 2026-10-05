pub open spec fn protocol_message_attributes__3_6_2_4_spec(message_id: UInt32, status: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
  (!IsMessageImplemented(old_s, 1, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsMessageAvailableToAgent(old_s, 1, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> attributes == 0)
  && ((!(IsMessageImplemented(old_s, 1, message_id)) &&
       IsMessageImplemented(old_s, 1, message_id))
    ==> ResultEqual(status, SUCCESS))
}