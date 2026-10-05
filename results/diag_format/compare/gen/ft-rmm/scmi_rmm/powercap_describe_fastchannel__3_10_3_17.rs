pub open spec fn powercap_describe_fastchannel__3_10_3_17_spec(domain_id: UInt32, message_id: UInt32, cpli: UInt32, status: Int32, attributes: UInt32, rate_limit: UInt32, chan_addr_low: UInt32, chan_addr_high: UInt32, chan_size: UInt32, doorbell_addr_low: UInt32, doorbell_addr_high: UInt32, doorbell_set_mask_low: UInt32, doorbell_set_mask_high: UInt32, doorbell_preserve_mask_low: UInt32, doorbell_preserve_mask_high: UInt32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidMessage(old_s, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidCpli(old_s, domain_id, cpli) ==> ResultEqual(status, NOT_FOUND))
  && (!DomainSupportsFastChannel(old_s, domain_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!MessageSupportsFastChannel(old_s, message_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (result == RSI_SUCCESS ==> ResultEqual(status, SUCCESS))
  && (result == RSI_SUCCESS ==> IsValidFastChannel(new_s, domain_id, message_id, cpli))
  && (result == RSI_SUCCESS ==> attributes[31..3] == 0)
  && (result == RSI_SUCCESS ==> attributes[0] == FastChannelHasDoorbell(new_s, domain_id, message_id, cpli))
  && (result == RSI_SUCCESS ==> attributes[0] == 1 ==> attributes[2..1] == DoorbellWidthEncoding(new_s, domain_id, message_id, cpli))
  && (result == RSI_SUCCESS ==> rate_limit[31..20] == 0)
  && (result == RSI_SUCCESS ==> (chan_addr_high << 32 | chan_addr_low) == FastChannelAddress(new_s, domain_id, message_id, cpli))
  && (result == RSI_SUCCESS ==> chan_size == FastChannelSize(new_s, domain_id, message_id, cpli))
  && (result == RSI_SUCCESS ==> attributes[0] == 1 ==> (doorbell_addr_high << 32 | doorbell_addr_low) == DoorbellAddress(new_s, domain_id, message_id, cpli))
  && ((IsValidPowercapDomain(old_s, domain_id) &&
       IsValidMessage(old_s, message_id) &&
       IsValidCpli(old_s, domain_id, cpli) &&
       DomainSupportsFastChannel(old_s, domain_id) &&
       MessageSupportsFastChannel(old_s, message_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != RSI_SUCCESS
    ==> IsValidFastChannel(new_s, domain_id, message_id, cpli))
  && (result != RSI_SUCCESS
    ==> attributes[31..3] == 0)
  && (result != RSI_SUCCESS
    ==> attributes[0] == FastChannelHasDoorbell(new_s, domain_id, message_id, cpli))
  && (result != RSI_SUCCESS
    ==> attributes[0] == 1 ==> attributes[2..1] == DoorbellWidthEncoding(new_s, domain_id, message_id, cpli))
  && (result != RSI_SUCCESS
    ==> rate_limit[31..20] == 0)
  && (result != RSI_SUCCESS
    ==> (chan_addr_high << 32 | chan_addr_low) == FastChannelAddress(new_s, domain_id, message_id, cpli))
  && (result != RSI_SUCCESS
    ==> chan_size == FastChannelSize(new_s, domain_id, message_id, cpli))
  && (result != RSI_SUCCESS
    ==> attributes[0] == 1 ==> (doorbell_addr_high << 32 | doorbell_addr_low) == DoorbellAddress(new_s, domain_id, message_id, cpli))
}