pub open spec fn performance_domain_name_get__3_5_6_19_spec(old_s: S, new_s: S, domain_id: UInt32, status: i32, flags: UInt32, name: Seq<u8>) -> bool {
    (!PerfDomainExists(old_s, domain_id) ==> status == NOT_FOUND)
    && ((status == SUCCESS) ==> (
        PerfDomainExists(old_s, domain_id)
        && PerfDomainExtendedNameSupported(old_s, domain_id)
        && flags == 0
        && name.len() == 64
        && IsNullTerminatedAscii(name)
        && name == PerfDomainExtendedName(old_s, domain_id)
    ))
    && new_s == old_s
}
