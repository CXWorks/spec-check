pub open spec fn sbi_mpxy_write_attributes_spec(result: u64, old_s: S, new_s: S, channel_id: u32, base_attribute_id: u32, attribute_count: u32) -> bool {
    (result == 0)
    && (forall i: u32 where i < attribute_count:
        ChannelAttribute(channel_id, base_attribute_id + i) == SharedMemWord(CallingHart(), 4 * i))
}