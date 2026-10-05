pub open spec fn power_domain_attributes__3_3_2_5_spec(result: Int32, attributes: UInt32, name: UInt8[16], old_s: S, new_s: S) -> bool {
    (!PowerDomainExists(Bits(old_s.domain_id, 15, 0)) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (
        (Bits(attributes, 31, 31) == 1) == PowerStateChangeNotifySupported(Bits(old_s.domain_id, 15, 0))
        && (Bits(attributes, 30, 30) == 1) == PowerStateAsyncSetSupported(Bits(old_s.domain_id, 15, 0))
        && (Bits(attributes, 29, 29) == 1) == PowerStateSyncSetSupported(Bits(old_s.domain_id, 15, 0))
        && (Bits(attributes, 28, 28) == 1) == PowerStateChangeRequestedNotifySupported(Bits(old_s.domain_id, 15, 0))
        && (Bits(attributes, 27, 27) == 1) == (PowerDomainNameLength(Bits(old_s.domain_id, 15, 0)) > 16)
        && Bits(attributes, 26, 0) == 0
        && (Bits(attributes, 27, 27) == 0 ==> (IsNullTerminatedAscii(name, 16) && name == PowerDomainName(Bits(old_s.domain_id, 15, 0))))
        && (Bits(attributes, 27, 27) == 1 ==> name == NullTerminatedPrefix(PowerDomainName(Bits(old_s.domain_id, 15, 0)), 15))
    ))
}