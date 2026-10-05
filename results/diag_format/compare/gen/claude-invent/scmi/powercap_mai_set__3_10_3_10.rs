pub open spec fn powercap_mai_set__3_10_3_10_spec(domain_id: u32, flags: u32, mai: u32, status: i32, old_s: S, new_s: S) -> bool {
    (!PowercapDomainExists(old_s, domain_id) ==> status == NOT_FOUND)
    && ((PowercapDomainExists(old_s, domain_id)
        && !PowercapMaiConfigSupported(old_s, domain_id)) ==> status == NOT_SUPPORTED)
    && ((PowercapDomainExists(old_s, domain_id)
        && PowercapMaiConfigSupported(old_s, domain_id)
        && (flags != 0 || mai == 0 || !PowercapMaiSupported(old_s, domain_id, mai))) ==> status == INVALID_PARAMETERS)
    && ((PowercapDomainExists(old_s, domain_id)
        && PowercapMaiConfigSupported(old_s, domain_id)
        && flags == 0
        && mai != 0
        && PowercapMaiSupported(old_s, domain_id, mai)
        && !PowercapAgentAllowedSetMai(old_s, domain_id)) ==> status == DENIED)
    && ((PowercapDomainExists(old_s, domain_id)
        && PowercapMaiConfigSupported(old_s, domain_id)
        && flags == 0
        && mai != 0
        && PowercapMaiSupported(old_s, domain_id, mai)
        && PowercapAgentAllowedSetMai(old_s, domain_id)) ==> (status == SUCCESS
            && PowercapDomainMai(new_s, domain_id) == mai
            && PowercapStateUnchangedExceptMai(old_s, new_s, domain_id)))
    && (status != SUCCESS ==> new_s == old_s)
}
