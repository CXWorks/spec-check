pub open spec fn sbi_mpxy_write_attributes_spec(channel_id: uint32_t, base_attribute_id: uint32_t, attribute_count: uint32_t, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_SUCCESS ==> (forall (i: UInt32), (i < attribute_count) ==> (ChannelAttribute(new_s, channel_id, base_attribute_id + i) == SharedMemoryWord(new_s, 4 * i))))
  && ((!(result == SBI_SUCCESS))
    ==> (ChannelAttribute(new_s, channel_id, base_attribute_id) == ChannelAttribute(old_s, channel_id, base_attribute_id)))
  && ((!(result == SBI_SUCCESS))
    ==> (ChannelAttribute(new_s, channel_id, base_attribute_id + 1) == ChannelAttribute(old_s, channel_id, base_attribute_id + 1)))
  && ((!(result == SBI_SUCCESS))
    ==> (ChannelAttribute(new_s, channel_id, base_attribute_id + 2) == ChannelAttribute(old_s, channel_id, base_attribute_id + 2)))
}