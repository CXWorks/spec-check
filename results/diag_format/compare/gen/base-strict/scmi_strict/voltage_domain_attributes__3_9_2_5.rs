pub open spec fn voltage_domain_attributes__3_9_2_5_spec(result: Int32, attributes: UInt32, name: UInt8[16], old_s: S, new_s: S) -> bool {
    (!VoltageDomainExists(Bits(old_s.domain_id, 15, 0)) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (
        ResultEqual(result, SUCCESS)
        && ((Bit(attributes, 31) == 1) == VoltageDomainSupportsAsyncLevelSet(Bits(old_s.domain_id, 15, 0)))
        && ((Bit(attributes, 30) == 1) == (VoltageDomainNameLength(Bits(old_s.domain_id, 15, 0)) > 16))
        && (Bits(attributes, 29, 0) == 0)
        && (Bit(attributes, 30) == 0 ==> NameEquals(name, VoltageDomainName(Bits(old_s.domain_id, 15, 0))))
        && (Bit(attributes, 30) == 1 ==> NameEqualsLowerBytes(name, VoltageDomainName(Bits(old_s.domain_id, 15, 0)), 15))
    ))
}