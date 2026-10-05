pub open spec fn protocol_message_attributes__3_10_3_4_spec(status: Int32, attributes: UInt32, message_id: UInt32, old_s: S, new_s: S) -> bool {
    (!IsMessageImplemented(message_id) ==> ResultEqual(status, NOT_FOUND))
    && ((message_id == POWERCAP_CAP_NOTIFY || message_id == POWERCAP_MEASUREMENTS_NOTIFY) && !IsNotificationAvailableToAgent(message_id) ==> ResultEqual(status, NOT_FOUND))
    && ResultEqual(status, SUCCESS)
    && (attributes & 0xFFFFFFFE) == 0
    && (attributes & 1) == (HasDedicatedFastChannel(message_id) as int)
    && ((message_id == POWERCAP_CAP_NOTIFY || message_id == POWERCAP_MEASUREMENTS_NOTIFY) ==> IsNotificationSupported(message_id))
}