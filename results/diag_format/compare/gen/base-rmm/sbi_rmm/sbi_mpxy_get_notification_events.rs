pub open spec fn sbi_mpxy_get_notification_events_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (ChannelSupportsEventsState(old_s, channel_id) && EventsStateEnabled(old_s, channel_id) ==>
        ShmemWord32(old_s, channel_id, 0x0) == EventsRemaining(channel_id) &&
        ShmemWord32(old_s, channel_id, 0x4) == EventsReturned(channel_id) &&
        ShmemWord32(old_s, channel_id, 0x8) == EventsLost(channel_id) &&
        ShmemNotificationData(old_s, channel_id, 0x10) == ProtocolNotificationEvents(channel_id) &&
        result == sbiret { error: 0, value: () })
}