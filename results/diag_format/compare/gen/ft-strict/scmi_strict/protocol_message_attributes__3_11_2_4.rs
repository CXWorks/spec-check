pub open spec fn protocol_message_attributes__3_11_2_4_spec(message_id: UInt32, status: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
  (!IsMessageValid(0x19, message_id) || !IsMessageImplemented(0x19, message_id) || !IsMessageAvailableToCallingAgent(0x19, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> IsMessageImplemented(0x19, message_id) && IsMessageAvailableToCallingAgent(0x19, message_id))
  && (ResultEqual(status, SUCCESS) ==> attributes == 0)
  && ((!(IsMessageValid(0x19, message_id) || !IsMessageImplemented(0x19, message_id) || !IsMessageAvailableToCallingAgent(0x19, message_id)))
    ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS)
    ==> IsMessageImplemented(0x19, message_id) && IsMessageAvailableToCallingAgent(0x19, message_id))
  && (ResultEqual(status, SUCCESS)
    ==> attributes == 0)
}