pub open spec fn sbi_mpxy_read_attributes_spec(channel_id: UInt32, base_attribute_id: UInt32, attribute_count: UInt32, result: SbiRet, old_s: S, new_s: S) -> bool {
  (result == SbiRet { code: 0, union: () } ==> AttributesRead(new_s, channel_id, base_attribute_id, attribute_count))
}