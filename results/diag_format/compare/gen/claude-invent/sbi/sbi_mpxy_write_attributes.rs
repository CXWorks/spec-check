pub open spec fn sbi_mpxy_write_attributes_spec(channel_id: UInt32, base_attribute_id: UInt32, attribute_count: UInt32, result: SbiRet, old_s: S, new_s: S) -> bool {
    (!MpxyShmemEnabled(old_s) ==> result.error == SBI_ERR_NO_SHMEM)
    && ((MpxyShmemEnabled(old_s) && !MpxyChannelIsValid(old_s, channel_id)) ==> result.error == SBI_ERR_NOT_SUPPORTED)
    && ((MpxyShmemEnabled(old_s) && MpxyChannelIsValid(old_s, channel_id)
         && (attribute_count == 0 || !MpxyAttributeIdIsValid(old_s, channel_id, base_attribute_id)))
        ==> result.error == SBI_ERR_INVALID_PARAM)
    && ((MpxyShmemEnabled(old_s) && MpxyChannelIsValid(old_s, channel_id)
         && attribute_count != 0 && MpxyAttributeIdIsValid(old_s, channel_id, base_attribute_id)
         && !MpxyAttributeRangeIsValid(old_s, channel_id, base_attribute_id, attribute_count))
        ==> result.error == SBI_ERR_BAD_RANGE)
    && ((MpxyShmemEnabled(old_s) && MpxyChannelIsValid(old_s, channel_id)
         && attribute_count != 0 && MpxyAttributeIdIsValid(old_s, channel_id, base_attribute_id)
         && MpxyAttributeRangeIsValid(old_s, channel_id, base_attribute_id, attribute_count)
         && MpxyAttributeRangeHasReadOnly(old_s, channel_id, base_attribute_id, attribute_count))
        ==> result.error == SBI_ERR_DENIED)
    && ((result.error != SBI_SUCCESS) ==> MpxyChannelAttributesUnchanged(old_s, new_s, channel_id))
    && ((MpxyShmemEnabled(old_s) && MpxyChannelIsValid(old_s, channel_id)
         && attribute_count != 0 && MpxyAttributeIdIsValid(old_s, channel_id, base_attribute_id)
         && MpxyAttributeRangeIsValid(old_s, channel_id, base_attribute_id, attribute_count)
         && !MpxyAttributeRangeHasReadOnly(old_s, channel_id, base_attribute_id, attribute_count))
        ==> (result.error == SBI_SUCCESS
             && (forall|i: int| 0 <= i < attribute_count as int ==>
                    MpxyChannelAttribute(new_s, channel_id, (base_attribute_id as int) + i)
                        == MpxyShmemReadU32(old_s, 4 * i))
             && (forall|a: int| !((base_attribute_id as int) <= a < (base_attribute_id as int) + (attribute_count as int)) ==>
                    MpxyChannelAttribute(new_s, channel_id, a) == MpxyChannelAttribute(old_s, channel_id, a))))
}
