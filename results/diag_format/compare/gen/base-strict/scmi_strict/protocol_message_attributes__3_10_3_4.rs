pub open spec fn protocol_message_attributes__3_10_3_4_spec(status: int, attributes: uint32, message_id: uint32, old_s: S, new_s: S) -> bool {
    (!IsValidMessage(message_id) ==> ResultEqual(status, NOT_FOUND))
    && (!IsImplementedMessage(message_id) ==> ResultEqual(status, NOT_FOUND))
    && (message_id == POWERCAP_CAP_NOTIFY && (!NotificationsImplemented(message_id) || !NotificationsAvailableToAgent(message_id, calling_agent)) ==> ResultEqual(status, NOT_FOUND))
    && (message_id == POWERCAP_MEASUREMENTS_NOTIFY && (!NotificationsImplemented(message_id) || !NotificationsAvailableToAgent(message_id, calling_agent)) ==> ResultEqual(status, NOT_FOUND))
    && (ResultEqual(status, SUCCESS) ==> IsImplementedMessage(message_id))
    && (ResultEqual(status, SUCCESS) ==> IsAvailableToAgent(message_id, calling_agent))
    && (message_id == POWERCAP_CAP_NOTIFY ==> ResultEqual(status, SUCCESS) ==> NotificationsSupported(POWERCAP_CAP_NOTIFY, calling_agent))
    && (message_id == POWERCAP_MEASUREMENTS_NOTIFY ==> ResultEqual(status, SUCCESS) ==> NotificationsSupported(POWERCAP_MEASUREMENTS_NOTIFY, calling_agent))
    && (ResultEqual(status, SUCCESS) ==> Bits(attributes, 31, 1) == 0)
    && (HasDedicatedFastChannel(message_id) ==> ResultEqual(status, SUCCESS) ==> Bits(attributes, 0, 0) == 1)
    && (!HasDedicatedFastChannel(message_id) ==> ResultEqual(status, SUCCESS) ==> Bits(attributes, 0, 0) == 0)
    && (old_s == new_s)
}