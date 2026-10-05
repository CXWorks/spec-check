pub open spec fn powercap_describe_fastchannel__3_10_3_17_spec(
    result: i32,
    domain_id: UInt32,
    message_id: UInt32,
    cpli: UInt32,
    old_s: S,
    new_s: S,
) -> bool {
    (!IsValidPowercapDomain(domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidMessage(message_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidCpli(domain_id, cpli) ==> ResultEqual(result, NOT_FOUND))
    && (!DomainSupportsFastChannel(domain_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!MessageSupportsFastChannel(message_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> IsValidFastChannel(domain_id, message_id, cpli))
    && (ResultEqual(result, SUCCESS) ==> (attributes[31:3] == 0))
    && (ResultEqual(result, SUCCESS) ==> (attributes[0] == FastChannelHasDoorbell(domain_id, message_id, cpli)))
    && (ResultEqual(result, SUCCESS) ==> (attributes[0] == 1 ==> attributes[2:1] == DoorbellWidthEncoding(domain_id, message_id, cpli)))
    && (ResultEqual(result, SUCCESS) ==> (rate_limit[31:20] == 0))
    && (ResultEqual(result, SUCCESS) ==> ((chan_addr_high << 32 | chan_addr_low) == FastChannelAddress(domain_id, message_id, cpli)))
    && (ResultEqual(result, SUCCESS) ==> (chan_size == FastChannelSize(domain_id, message_id, cpli)))
    && (ResultEqual(result, SUCCESS) ==> (attributes[0] == 1 ==> (doorbell_addr_high << 32 | doorbell_addr_low) == DoorbellAddress(domain_id, message_id, cpli)))
}