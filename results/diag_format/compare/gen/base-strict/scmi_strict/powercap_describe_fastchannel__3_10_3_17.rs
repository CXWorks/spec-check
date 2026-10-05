pub open spec fn powercap_describe_fastchannel__3_10_3_17_spec(
    result: Result<(), RmiStatusCode>,
    old_s: S,
    new_s: S,
    protocol_id: UInt8,
    message_id: UInt8,
    domain_id: UInt32,
    message_id_payload: UInt32,
    cpli: UInt32,
    status: Int32,
    attributes: UInt32,
    rate_limit: UInt32,
    chan_addr_low: UInt32,
    chan_addr_high: UInt32,
    chan_size: UInt32,
    doorbell_addr_low: UInt32,
    doorbell_addr_high: UInt32,
    doorbell_set_mask_low: UInt32,
    doorbell_set_mask_high: UInt32,
    doorbell_preserve_mask_low: UInt32,
    doorbell_preserve_mask_high: UInt32,
) -> bool {
    (!IsValidPowercapDomain(domain_id) ==> ResultEqual(result, RMI_ERROR_NOT_FOUND))
    && (!IsValidPowercapMessage(message_id_payload) ==> ResultEqual(result, RMI_ERROR_NOT_FOUND))
    && (!IsValidCpli(domain_id, cpli) ==> ResultEqual(result, RMI_ERROR_NOT_FOUND))
    && (!DomainSupportsFastChannel(domain_id) || !MessageSupportsFastChannel(message_id_payload) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (ResultEqual(result, RMI_SUCCESS) ==> (
        ResultEqual(result, RMI_SUCCESS)
        && Bits(attributes, 31, 3) == 0
        && (Bits(attributes, 0, 0) == 1) == FastChannelHasDoorbell(domain_id, message_id_payload, cpli)
        && (Bits(attributes, 0, 0) == 1 && Bits(attributes, 2, 1) == 0 ==> DoorbellWidth(domain_id, message_id_payload, cpli) == 8)
        && (Bits(attributes, 0, 0) == 1 && Bits(attributes, 2, 1) == 1 ==> DoorbellWidth(domain_id, message_id_payload, cpli) == 16)
        && (Bits(attributes, 0, 0) == 1 && Bits(attributes, 2, 1) == 2 ==> DoorbellWidth(domain_id, message_id_payload, cpli) == 32)
        && (Bits(attributes, 0, 0) == 1 && Bits(attributes, 2, 1) == 3 ==> DoorbellWidth(domain_id, message_id_payload, cpli) == 64)
        && Bits(rate_limit, 31, 20) == 0
        && Bits(rate_limit, 19, 0) == FastChannelRateLimitUs(domain_id, message_id_payload, cpli)
        && chan_addr_low == Bits(FastChannelAddress(domain_id, message_id_payload, cpli), 31, 0)
        && chan_addr_high == Bits(FastChannelAddress(domain_id, message_id_payload, cpli), 63, 32)
        && chan_size == FastChannelSizeBytes(domain_id, message_id_payload, cpli)
        && Bits(attributes, 0, 0) == 1 ==> (
            doorbell_addr_low == Bits(DoorbellAddress(domain_id, message_id_payload, cpli), 31, 0)
            && doorbell_addr_high == Bits(DoorbellAddress(domain_id, message_id_payload, cpli), 63, 32)
        )
        && Bits(attributes, 0, 0) == 1 ==> (
            doorbell_set_mask_low == Bits(DoorbellSetMask(domain_id, message_id_payload, cpli), 31, 0)
            && (DoorbellWidth(domain_id, message_id_payload, cpli) == 64 ==> doorbell_set_mask_high == Bits(DoorbellSetMask(domain_id, message_id_payload, cpli), 63, 32))
        )
        && Bits(attributes, 0, 0) == 1 ==> (
            doorbell_preserve_mask_low == Bits(DoorbellPreserveMask(domain_id, message_id_payload, cpli), 31, 0)
            && (DoorbellWidth(domain_id, message_id_payload, cpli) == 64 ==> doorbell_preserve_mask_high == Bits(DoorbellPreserveMask(domain_id, message_id_payload, cpli), 63, 32))
        )
        && Bits(attributes, 0, 0) == 1 ==> DoorbellWriteClearsBitsNotInSetOrPreserveMask(domain_id, message_id_payload, cpli)
    ))
}