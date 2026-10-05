pub open spec fn sbi_mpxy_write_attributes_spec(channel_id: uint32, base_attribute_id: uint32, attribute_count: uint32, result: SbiReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_SUCCESS ==> (forall i in [0, attribute_count): ChannelAttribute(new_s, channel_id, base_attribute_id + i) == SharedMemWord(new_s, CallingHart(new_s), 4 * i)))
  && ((!(result == SBI_SUCCESS))
    ==> (ChannelAttribute(new_s, channel_id, base_attribute_id) == ChannelAttribute(old_s, channel_id, base_attribute_id)))
}