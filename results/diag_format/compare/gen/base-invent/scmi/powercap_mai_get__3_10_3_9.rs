pub open spec fn powercap_mai_get__3_10_3_9_spec(result: int32, mai: uint32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> !DomainExists(old_s, domain_id))
    && (result == NOT_SUPPORTED ==> !PowerCapDomainSupported(old_s, domain_id))
    && (result == SUCCESS ==> (DomainExists(old_s, domain_id) && PowerCapDomainSupported(old_s, domain_id)))
    && (result == SUCCESS ==> (old_s.powercap_mai(domain_id) == mai))
    && (result == SUCCESS ==> (new_s.powercap_mai(domain_id) == old_s.powercap_mai(domain_id)))
}