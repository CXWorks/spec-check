pub open spec fn voltage_domain_name_get__3_9_2_11_spec(domain_id: u32, status: i32, flags: u32, name: Seq<u8>, old_s: S, new_s: S) -> bool {
    (!VoltageDomainExists(old_s, domain_id) ==> status == NOT_FOUND)
    && (status == SUCCESS ==> (
        VoltageDomainExists(old_s, domain_id)
        && VoltageDomainExtendedNameSupported(old_s, domain_id)
        && flags == 0
        && name.len() == 64
        && IsNullTerminatedAsciiString(name, 64)
        && name == VoltageDomainExtendedName(old_s, domain_id)
    ))
    && new_s == old_s
}
