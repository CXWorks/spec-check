pub open spec fn powercap_domain_name_get__3_10_3_13_spec(status: i32, flags: u32, name: Seq<u8>, domain_id: u32, old_s: S, new_s: S) -> bool {
    (!PowercapDomainExists(old_s, domain_id) ==> status == NOT_FOUND)
    && (status == SUCCESS ==> (
        PowercapDomainExists(old_s, domain_id)
        && PowercapDomainExtendedNameSupported(old_s, domain_id)
        && flags == 0
        && name.len() == 64
        && IsNullTerminatedAsciiString(name, 64)
        && name == PowercapDomainExtendedName(old_s, domain_id)
    ))
    && new_s == old_s
}
