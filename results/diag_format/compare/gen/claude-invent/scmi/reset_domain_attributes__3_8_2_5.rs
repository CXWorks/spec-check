pub open spec fn reset_domain_attributes__3_8_2_5_spec(domain_id: UInt32, status: Int32, attributes: UInt32, latency: UInt32, name: Seq<u8>, old_s: S, new_s: S) -> bool {
    (!ResetDomainExists(old_s, domain_id) ==> (status == NOT_FOUND && new_s == old_s))
    && (ResetDomainExists(old_s, domain_id) ==> (
        status == SUCCESS
        && (((attributes >> 31u32) & 1u32) == 1u32 <==> ResetDomainSupportsAsyncReset(old_s, domain_id))
        && (((attributes >> 30u32) & 1u32) == 1u32 <==> ResetDomainSupportsResetNotifications(old_s, domain_id))
        && (((attributes >> 29u32) & 1u32) == 1u32 <==> ResetDomainHasExtendedName(old_s, domain_id))
        && ((attributes & 0x1FFF_FFFFu32) == 0u32)
        && (ResetDomainLatencySupported(old_s, domain_id) ==> latency == ResetDomainMaxLatency(old_s, domain_id))
        && (!ResetDomainLatencySupported(old_s, domain_id) ==> latency == 0xFFFF_FFFFu32)
        && name.len() == 16
        && IsNullTerminatedAsciiString(name)
        && (!ResetDomainHasExtendedName(old_s, domain_id) ==> name == ResetDomainName(old_s, domain_id))
        && (ResetDomainHasExtendedName(old_s, domain_id) ==> (
            name.subrange(0, 15) == ResetDomainName(old_s, domain_id).subrange(0, 15)
            && name[15] == 0u8
        ))
        && new_s == old_s
    ))
}
