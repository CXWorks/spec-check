pub open spec fn sbi_mpxy_get_notification_events_spec(channel_id: UInt32, old_s: S, new_s: S) -> bool {
  (ChannelSupportsEventsState(old_s, channel_id) && EventsStateEnabled(old_s, channel_id) ==> ShmemWord32(new_s, channel_id, 0x0) == EventsRemaining(new_s, channel_id))
  && (ChannelSupportsEventsState(old_s, channel_id) && EventsStateEnabled(old_s, channel_id) ==> ShmemWord32(new_s, channel_id, 0x4) == EventsReturned(new_s, channel_id))
  && (ChannelSupportsEventsState(old_s, channel_id) && EventsStateEnabled(old_s, channel_id) ==> ShmemWord32(new_s, channel_id, 0x8) == EventsLost(new_s, channel_id))
  && (ChannelSupportsEventsState(old_s, channel_id) && EventsStateEnabled(old_s, channel_id) ==> ShmemNotificationData(new_s, channel_id, 0x10) == ProtocolNotificationEvents(new_s, channel_id))
  && ((!(ChannelSupportsEventsState(old_s, channel_id) && EventsStateEnabled(old_s, channel_id)))
    ==> ShmemWord32(new_s, channel_id, 0x0) == ShmemWord32(old_s, channel_id, 0x0))
  && ((!(ChannelSupportsEventsState(old_s, channel_id) && EventsStateEnabled(old_s, channel_id)))
    ==> ShmemWord32(new_s, channel_id, 0x4) == ShmemWord32(old_s, channel_id, 0x4))
  && ((!(ChannelSupportsEventsState(old_s, channel_id) && EventsStateEnabled(old_s, channel_id)))
    ==> ShmemWord32(new_s, channel_id, 0x8) == ShmemWord32(old_s, channel_id, 0x8))
  && ((!(ChannelSupportsEventsState(old_s, channel_id) && EventsStateEnabled(old_s, channel_id)))
    ==> ShmemNotificationData(new_s, channel_id, 0x10) == ShmemNotificationData(old_s, channel_id, 0x10))
}