pub open spec fn powercap_mai_get__3_10_3_9_spec(domain_id: u32, status: i32, mai: u32, old_s: S, new_s: S) -> bool {
    (!IsValidPowercapDomain(old_s, domain_id) ==> status == NOT_FOUND)
    && ((IsValidPowercapDomain(old_s, domain_id) && !PowercapMaiGetSupported(old_s, domain_id)) ==> status == NOT_SUPPORTED)
    && ((IsValidPowercapDomain(old_s, domain_id) && PowercapMaiGetSupported(old_s, domain_id)) ==> (status == SUCCESS && mai == PowercapDomainMai(old_s, domain_id)))
    && (status == SUCCESS ==> (IsValidPowercapDomain(old_s, domain_id) && PowercapMaiGetSupported(old_s, domain_id) && mai == PowercapDomainMai(old_s, domain_id)))
    && (new_s == old_s)
}
