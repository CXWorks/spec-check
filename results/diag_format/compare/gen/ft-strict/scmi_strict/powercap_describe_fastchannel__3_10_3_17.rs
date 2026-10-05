pub open spec fn powercap_describe_fastchannel__3_10_3_17_spec(domain_id: UInt32, message_id: UInt32, cpli: UInt32, status: Int32, attributes: UInt32, rate_limit: UInt32, chan_addr_low: UInt32, chan_addr_high: UInt32, chan_size: UInt32, doorbell_addr_low: UInt32, doorbell_addr_high: UInt32, doorbell_set_mask_low: UInt32, doorbell_set_mask_high: UInt32, doorbell_preserve_mask_low: UInt32, doorbell_preserve_mask_high: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidPowercapMessage(old_s, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidCpli(old_s, domain_id, cpli) ==> ResultEqual(status, NOT_FOUND))
  && (!DomainSupportsFastChannel(old_s, domain_id) || !MessageSupportsFastChannel(old_s, message_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> Bits(attributes, 31, 3) == 0)
  && (ResultEqual(status, SUCCESS) ==> (Bits(attributes, 0, 0) == 1) == FastChannelHasDoorbell(old_s, domain_id, message_id, cpli))
  && (ResultEqual(status, SUCCESS) ==> Bits(attributes, 0, 0) == 1 && Bits(attributes, 2, 1) == 0 ==> DoorbellWidth(old_s, domain_id, message_id, cpli) == 8)
  && (ResultEqual(status, SUCCESS) ==> Bits(attributes, 0, 0) == 1 && Bits(attributes, 2, 1) == 1 ==> DoorbellWidth(old_s, domain_id, message_id, cpli) == 16)
  && (ResultEqual(status, SUCCESS) ==> Bits(attributes, 0, 0) == 1 && Bits(attributes, 2, 1) == 2 ==> DoorbellWidth(old_s, domain_id, message_id, cpli) == 32)
  && (ResultEqual(status, SUCCESS) ==> Bits(attributes, 0, 0) == 1 && Bits(attributes, 2, 1) == 3 ==> DoorbellWidth(old_s, domain_id, message_id, cpli) == 64)
  && (ResultEqual(status, SUCCESS) ==> Bits(rate_limit, 31, 20) == 0)
  && (ResultEqual(status, SUCCESS) ==> Bits(rate_limit, 19, 0) == FastChannelRateLimitUs(old_s, domain_id, message_id, cpli))
  && (ResultEqual(status, SUCCESS) ==> chan_addr_low == Bits(FastChannelAddress(old_s, domain_id, message_id, cpli), 31, 0) && chan_addr_high == Bits(FastChannelAddress(old_s, domain_id, message_id, cpli), 63, 32))
  && (ResultEqual(status, SUCCESS) ==> chan_size == FastChannelSizeBytes(old_s, domain_id, message_id, cpli))
  && (ResultEqual(status, SUCCESS) ==> Bits(attributes, 0, 0) == 1 ==> doorbell_addr_low == Bits(DoorbellAddress(old_s, domain_id, message_id, cpli), 31, 0) && doorbell_addr_high == Bits(DoorbellAddress(old_s, domain_id, message_id, cpli), 63, 32))
  && (ResultEqual(status, SUCCESS) ==> Bits(attributes, 0, 0) == 1 ==> doorbell_set_mask_low == Bits(DoorbellSetMask(old_s, domain_id, message_id, cpli), 31, 0) && (DoorbellWidth(old_s, domain_id, message_id, cpli) == 64 ==> doorbell_set_mask_high == Bits(DoorbellSetMask(old_s, domain_id, message_id, cpli), 63, 32)))
  && (ResultEqual(status, SUCCESS) ==> Bits(attributes, 0, 0) == 1 ==> doorbell_preserve_mask_low == Bits(DoorbellPreserveMask(old_s, domain_id, message_id, cpli), 31, 0) && (DoorbellWidth(old_s, domain_id, message_id, cpli) == 64 ==> doorbell_preserve_mask_high == Bits(DoorbellPreserveMask(old_s, domain_id, message_id, cpli), 63, 32)))
  && (ResultEqual(status, SUCCESS) ==> Bits(attributes, 0, 0) == 1 ==> DoorbellWriteClearsBitsNotInSetOrPreserveMask(old_s, domain_id, message_id, cpli))
  && ((IsValidPowercapDomain(old_s, domain_id) &&
       IsValidPowercapMessage(old_s, message_id) &&
       IsValidCpli(old_s, domain_id, cpli) &&
       (DomainSupportsFastChannel(old_s, domain_id) && MessageSupportsFastChannel(old_s, message_id)))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> Bits(attributes, 31, 3) == 0)
  && (result != SUCCESS
    ==> (Bits(attributes, 0, 0) == 1) == FastChannelHasDoorbell(old_s, domain_id, message_id, cpli))
  && (result != SUCCESS
    ==> Bits(attributes, 0, 0) == 1 && Bits(attributes, 2, 1) == 0 ==> DoorbellWidth(old_s, domain_id, message_id, cpli) == 8)
  && (result != SUCCESS
    ==> Bits(attributes, 0, 0) == 1 && Bits(attributes, 2, 1) == 1 ==> DoorbellWidth(old_s, domain_id, message_id, cpli) == 16)
  && (result != SUCCESS
    ==> Bits(attributes, 0, 0) == 1 && Bits(attributes, 2, 1) == 2 ==> DoorbellWidth(old_s, domain_id, message_id, cpli) == 32)
  && (result != SUCCESS
    ==> Bits(attributes, 0, 0) == 1 && Bits(attributes, 2, 1) == 3 ==> DoorbellWidth(old_s, domain_id, message_id, cpli) == 64)
  && (result != SUCCESS
    ==> Bits(rate_limit, 31, 20) == 0)
  && (result != SUCCESS
    ==> Bits(rate_limit, 19, 0) == FastChannelRateLimitUs(old_s, domain_id, message_id, cpli))
  && (result != SUCCESS
    ==> chan_addr_low == Bits(FastChannelAddress(old_s, domain_id, message_id, cpli), 31, 0) && chan_addr_high == Bits(FastChannelAddress(old_s, domain_id, message_id, cpli), 63, 32))
  && (result != SUCCESS
    ==> chan_size == FastChannelSizeBytes(old_s, domain_id, message_id, cpli))
  && (result != SUCCESS
    ==> Bits(attributes, 0, 0) == 1 ==> doorbell_addr_low == Bits(DoorbellAddress(old_s, domain_id, message_id, cpli), 31, 0) && doorbell_addr_high == Bits(DoorbellAddress(old_s, domain_id, message_id, cpli), 63, 32))
  && (result != SUCCESS
    ==> Bits(attributes, 0, 0) == 1 ==> doorbell_set_mask_low == Bits(DoorbellSetMask(old_s, domain_id, message_id, cpli), 31, 0) && (DoorbellWidth(old_s, domain_id, message_id, cpli) == 64 ==> doorbell_set_mask_high == Bits(DoorbellSetMask(old_s, domain_id, message_id, cpli), 63, 32)))
  && (result != SUCCESS
    ==> Bits(attributes, 0, 0) == 1 ==> doorbell_preserve_mask_low == Bits(DoorbellPreserveMask(old_s, domain_id, message_id, cpli), 31, 0) && (DoorbellWidth(old_s, domain_id, message_id, cpli) == 64 ==> doorbell_preserve_mask_high == Bits(DoorbellPreserveMask(old_s, domain_id, message_id, cpli), 63, 32)))
  && (result != SUCCESS
    ==> Bits(attributes, 0, 0) == 1 ==> DoorbellWriteClearsBitsNotInSetOrPreserveMask(old_s, domain_id, message_id, cpli))
}