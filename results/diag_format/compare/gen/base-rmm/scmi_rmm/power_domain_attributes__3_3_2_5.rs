pub open spec fn power_domain_attributes__3_3_2_5_spec(result: Int32, attributes: UInt32, name: UInt8[16], old_s: S, new_s: S) -> bool {
    (!PowerDomainExists(old_s, domain_id as int) ==> ResultEqual(result, NOT_FOUND))
    && (PowerDomainExists(old_s, domain_id as int) ==> (
        ResultEqual(result, SUCCESS)
        && (attributes[31] as int) == (PowerStateChangeNotifySupported(domain_id as int) ? 1 : 0)
        && (attributes[30] as int) == (PowerStateAsyncSetSupported(domain_id as int) ? 1 : 0)
        && (attributes[29] as int) == (PowerStateSyncSetSupported(domain_id as int) ? 1 : 0)
        && (attributes[28] as int) == (PowerStateChangeRequestedNotifySupported(domain_id as int) ? 1 : 0)
        && (attributes[27] as int) == (PowerDomainNameLength(domain_id as int) > 16 ? 1 : 0)
        && (attributes[26:0] as int) == 0
        && ((attributes[27] as int) == 0 ==> name == PowerDomainName(domain_id as int))
        && ((attributes[27] as int) == 1 ==> name == NullTerminated(PowerDomainName(domain_id as int)[14:0]))
    ))
    && (old_s == new_s)
}