pub open spec fn powercap_cap_get__3_10_3_7_spec(domain_id: u32, cpli: u32, status: i32, power_cap: u32, old_s: S, new_s: S) -> bool {
    ((!PowercapDomainIsValid(old_s, domain_id) || !PowercapCpliIsValid(old_s, domain_id, cpli)) ==> status == NOT_FOUND)
    && ((PowercapDomainIsValid(old_s, domain_id) && PowercapCpliIsValid(old_s, domain_id, cpli) && !PowercapCapGetIsSupported(old_s, domain_id)) ==> status == NOT_SUPPORTED)
    && (status == SUCCESS ==> (
        PowercapDomainIsValid(old_s, domain_id)
        && PowercapCpliIsValid(old_s, domain_id, cpli)
        && PowercapCapGetIsSupported(old_s, domain_id)
        && power_cap == PowercapEnforcedCap(old_s, domain_id, cpli)
    ))
    && new_s == old_s
}
