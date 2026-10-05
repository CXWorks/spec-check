pub open spec fn power_domain_name_get__3_3_2_10_spec(domain_id: UInt32, status: i32, flags: UInt32, ext_name: Seq<u8>, old_s: S, new_s: S) -> bool {
    (!PowerDomainExists(old_s, domain_id) ==> status == NOT_FOUND)
    && (status == SUCCESS ==> (
        PowerDomainExists(old_s, domain_id)
        && flags == 0
        && ext_name.len() == 64
        && (exists |i: int| 0 <= i < 64 && ext_name[i] == 0u8)
        && PowerDomainExtNameEqual(old_s, domain_id, ext_name)
    ))
    && new_s == old_s
}
