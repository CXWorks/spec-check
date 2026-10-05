pub open spec fn sbi_mpxy_write_attributes_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result == sbiret::ERR) ==> false
    && (result == sbiret::OK) ==> (forall|i: UInt32| (i < attribute_count(old_s) ==> ChannelAttribute(new_s, old_s.channel_id, old_s.base_attribute_id + i) == SharedMemoryWord(old_s.calling_hart, 4 * i)))
}