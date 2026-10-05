pub open spec fn protocol_message_attributes__3_10_3_4_spec(message_id: UInt32, status: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
  (!IsMessageImplemented(old_s, message_id) ==> ResultEqual(status, NOT_FOUND))
  && ((message_id == POWERCAP_CAP_NOTIFY || message_id == POWERCAP_MEASUREMENTS_NOTIFY) && !IsNotificationAvailableToAgent(old_s, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> attributes[31:1] == 0)
  && (ResultEqual(status, SUCCESS) ==> attributes[0] == (HasDedicatedFastChannel(old_s, message_id) ? 1 : 0))
  && ((message_id == POWERCAP_CAP_NOTIFY || message_id == POWERCAP_MEASUREMENTS_NOTIFY) ==> IsNotificationSupported(old_s, message_id))
  && ((!(IsMessageImplemented(old_s, message_id)) &&
       !((message_id == POWERCAP_CAP_NOTIFY || message_id == POWERCAP_MEASUREMENTS_NOTIFY) && !IsNotificationAvailableToAgent(old_s, message_id)))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> attributes[0] == 0)
}