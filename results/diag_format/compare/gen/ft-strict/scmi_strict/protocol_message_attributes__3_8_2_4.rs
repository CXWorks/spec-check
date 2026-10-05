pub open spec fn protocol_message_attributes__3_8_2_4_spec(message_id: uint32, status: int32, attributes: uint32, old_s: S, new_s: S) -> bool {
  (!IsMessageProvided(old_s, 0x16, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (result: Result<(), RmiStatusCode>, IsMessageProvided(old_s, 0x16, message_id) ==> ResultEqual(status, SUCCESS))
  && (result: Result<(), RmiStatusCode>, IsMessageProvided(old_s, 0x16, message_id) ==> IsMessageImplementedAndAvailable(old_s, 0x16, message_id))
  && (result: Result<(), RmiStatusCode>, IsMessageProvided(old_s, 0x16, message_id) ==> attributes == 0)
  && ((!(IsMessageProvided(old_s, 0x16, message_id)))
    ==> ResultEqual(status, SUCCESS))
  && (result: Result<(), RmiStatusCode>, result.is_Ok()
    ==> IsMessageImplementedAndAvailable(old_s, 0x16, message_id))
  && (result: Result<(), RmiStatusCode>, result.is_Ok()
    ==> attributes == 0)
}