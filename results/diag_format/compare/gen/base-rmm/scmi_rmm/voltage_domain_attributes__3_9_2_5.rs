pub open spec fn voltage_domain_attributes__3_9_2_5_spec(result: Int32, attributes: UInt32, name: UInt8[16], old_s: S, new_s: S) -> bool {
    (!VoltageDomainExists(old_s, domain_id as int) ==> ResultEqual(result, NOT_FOUND))
    && (VoltageDomainExists(old_s, domain_id as int) ==> (
        ResultEqual(result, SUCCESS)
        && (attributes as int) & (1u32 << 31) == (VoltageDomainSupportsAsyncLevelSet(old_s, domain_id as int) as int)
        && (attributes as int) & (1u32 << 30) == ((VoltageDomainNameLength(old_s, domain_id as int) > 16u32) as int)
        && ((attributes as int) & 0xFFFF_0000u32) == 0
        && IsNullTerminatedAscii(name, 16)
        && ((attributes as int) & (1u32 << 30) == 0) || (name == LowerBytes(VoltageDomainName(old_s, domain_id as int), 15))
    ))
}