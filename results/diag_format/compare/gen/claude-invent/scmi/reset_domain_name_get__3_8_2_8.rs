pub open spec fn reset_domain_name_get__3_8_2_8_spec(domain_id: u32, status: i32, flags: u32, name: Seq<u8>, old_s: S, new_s: S) -> bool {
    (!ResetDomainExists(old_s, domain_id) ==> status == NOT_FOUND)
    && (status == SUCCESS ==> (
        ResetDomainExists(old_s, domain_id)
        && ResetDomainExtendedNameSupported(old_s, domain_id)
        && flags == 0
        && name.len() == 64
        && (exists|i: int| 0 <= i < 64 && name[i] == 0u8)
        && name == ResetDomainExtendedName(old_s, domain_id)
    ))
    && (new_s == old_s)
}
