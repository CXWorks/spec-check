pub open spec fn voltage_domain_attributes__3_9_2_5_spec(domain_id: u32, status: i32, attributes: u32, name: Seq<u8>, old_s: S, new_s: S) -> bool {
    (!VoltageDomainExists(old_s, (domain_id & 0xFFFFu32)) ==> status == NOT_FOUND)
    && (status == SUCCESS ==> (
        VoltageDomainExists(old_s, (domain_id & 0xFFFFu32))
        && ((((attributes >> 31u32) & 1u32) == 1u32) == VoltageDomainAsyncLevelSetSupported(old_s, (domain_id & 0xFFFFu32)))
        && ((((attributes >> 30u32) & 1u32) == 1u32) == VoltageDomainExtendedNameSupported(old_s, (domain_id & 0xFFFFu32)))
        && ((attributes & 0x3FFF_FFFFu32) == 0u32)
        && name.len() == 16
        && (exists|i: int| 0 <= i < 16 && name[i] == 0u8)
        && (!VoltageDomainExtendedNameSupported(old_s, (domain_id & 0xFFFFu32)) ==> VoltageDomainNameEqual(old_s, (domain_id & 0xFFFFu32), name))
        && (VoltageDomainExtendedNameSupported(old_s, (domain_id & 0xFFFFu32)) ==> VoltageDomainExtendedNameLower15BytesEqual(old_s, (domain_id & 0xFFFFu32), name))
    ))
    && new_s == old_s
}
