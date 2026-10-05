pub open spec fn powercap_cai_get__3_10_3_11_spec(domain_id: u32, cpli: u32, status: i32, cai: u32, old_s: S, new_s: S) -> bool {
    (!IsValidPowercapDomain(old_s, domain_id) ==> status == NOT_FOUND)
    && ((IsValidPowercapDomain(old_s, domain_id) && !IsValidPowercapCpli(old_s, domain_id, cpli)) ==> status == NOT_FOUND)
    && ((IsValidPowercapDomain(old_s, domain_id) && !PowercapDomainSupportsCpc(old_s, domain_id) && cpli != 0) ==> status == NOT_FOUND)
    && ((IsValidPowercapDomain(old_s, domain_id) && IsValidPowercapCpli(old_s, domain_id, cpli) && !PowercapCaiGetSupported(old_s, domain_id)) ==> status == NOT_SUPPORTED)
    && (status == SUCCESS ==> (
        IsValidPowercapDomain(old_s, domain_id)
        && IsValidPowercapCpli(old_s, domain_id, cpli)
        && PowercapCaiGetSupported(old_s, domain_id)
        && cai == PowercapCaiOf(old_s, domain_id, cpli)
    ))
    && (new_s == old_s)
}
