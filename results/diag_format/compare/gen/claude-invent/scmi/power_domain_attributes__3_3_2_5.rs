pub open spec fn power_domain_attributes__3_3_2_5_spec(domain_id: UInt32, status: i32, attributes: UInt32, name: [u8; 16], old_s: S, new_s: S) -> bool {
    (!PowerDomainExists(old_s, (domain_id & 0xFFFFu32) as UInt32) ==> status == NOT_FOUND)
    && (status == SUCCESS ==> (
        PowerDomainExists(old_s, (domain_id & 0xFFFFu32) as UInt32)
        && ((((attributes >> 31u32) & 1u32) == 1u32) == PowerDomainStateChangeNotifySupported(old_s, (domain_id & 0xFFFFu32) as UInt32))
        && ((((attributes >> 30u32) & 1u32) == 1u32) == PowerDomainAsyncSetSupported(old_s, (domain_id & 0xFFFFu32) as UInt32))
        && ((((attributes >> 29u32) & 1u32) == 1u32) == PowerDomainSyncSetSupported(old_s, (domain_id & 0xFFFFu32) as UInt32))
        && ((((attributes >> 28u32) & 1u32) == 1u32) == PowerDomainStateChangeRequestedNotifySupported(old_s, (domain_id & 0xFFFFu32) as UInt32))
        && ((((attributes >> 27u32) & 1u32) == 1u32) == PowerDomainHasExtendedName(old_s, (domain_id & 0xFFFFu32) as UInt32))
        && ((attributes & 0x07FF_FFFFu32) == 0u32)
        && PowerDomainNameMatches(old_s, (domain_id & 0xFFFFu32) as UInt32, name, PowerDomainHasExtendedName(old_s, (domain_id & 0xFFFFu32) as UInt32))
    ))
    && (new_s == old_s)
}
