pub open spec fn powercap_cap_notify__3_10_3_15_spec(domain_id: u32, notify_enable: u32, status: i32, old_s: S, new_s: S) -> bool {
    (!PowercapDomainIsValid(old_s, domain_id) ==> status == NOT_FOUND)
    && ((PowercapDomainIsValid(old_s, domain_id) && (notify_enable >> 1u32) != 0u32) ==> status == INVALID_PARAMETERS)
    && (status == SUCCESS ==> (
        PowercapDomainIsValid(old_s, domain_id)
        && (notify_enable >> 1u32) == 0u32
        && PowercapCapNotifyEnabled(new_s, domain_id) == ((notify_enable & 1u32) == 1u32)
        && (forall|d: u32| d != domain_id ==> PowercapCapNotifyEnabled(new_s, d) == PowercapCapNotifyEnabled(old_s, d))
    ))
    && (status != SUCCESS ==> (
        forall|d: u32| PowercapCapNotifyEnabled(new_s, d) == PowercapCapNotifyEnabled(old_s, d)
    ))
}
