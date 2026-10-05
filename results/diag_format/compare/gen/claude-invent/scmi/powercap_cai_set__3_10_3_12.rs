pub open spec fn powercap_cai_set__3_10_3_12_spec(status: i32, old_s: S, new_s: S, domain_id: u32, flags: u32, cai: u32, cpli: u32) -> bool {
    ((!IsPowercapDomainValid(old_s, domain_id) || !IsPowercapCpliValid(old_s, domain_id, cpli)) ==> status != SUCCESS)
    && ((IsPowercapDomainValid(old_s, domain_id) && !PowercapDomainSupportsCpc(old_s, domain_id) && cpli != 0) ==> status != SUCCESS)
    && ((IsPowercapDomainValid(old_s, domain_id) && !PowercapDomainSupportsCaiConfig(old_s, domain_id)) ==> status != SUCCESS)
    && (flags != 0 ==> status != SUCCESS)
    && (cai == 0 ==> status != SUCCESS)
    && ((IsPowercapDomainValid(old_s, domain_id) && !IsPowercapCaiSupported(old_s, domain_id, cai)) ==> status != SUCCESS)
    && ((IsPowercapDomainValid(old_s, domain_id) && !IsAgentAllowedToSetPowercapCai(old_s, domain_id)) ==> status != SUCCESS)
    && (status == NOT_FOUND ==> (!IsPowercapDomainValid(old_s, domain_id) || !IsPowercapCpliValid(old_s, domain_id, cpli) || (!PowercapDomainSupportsCpc(old_s, domain_id) && cpli != 0)))
    && (status == NOT_SUPPORTED ==> !PowercapDomainSupportsCaiConfig(old_s, domain_id))
    && (status == INVALID_PARAMETERS ==> (flags != 0 || cai == 0 || !IsPowercapCaiSupported(old_s, domain_id, cai)))
    && (status == DENIED ==> !IsAgentAllowedToSetPowercapCai(old_s, domain_id))
    && (status != SUCCESS ==> new_s == old_s)
    && ((IsPowercapDomainValid(old_s, domain_id)
        && IsPowercapCpliValid(old_s, domain_id, cpli)
        && (PowercapDomainSupportsCpc(old_s, domain_id) || cpli == 0)
        && PowercapDomainSupportsCaiConfig(old_s, domain_id)
        && flags == 0
        && cai != 0
        && IsPowercapCaiSupported(old_s, domain_id, cai)
        && IsAgentAllowedToSetPowercapCai(old_s, domain_id))
        ==> (status == SUCCESS
            && PowercapCai(new_s, domain_id, cpli) == cai
            && PowercapStateUnchangedExceptCai(old_s, new_s, domain_id, cpli)))
    && (status == SUCCESS ==> (IsPowercapDomainValid(old_s, domain_id)
        && IsPowercapCpliValid(old_s, domain_id, cpli)
        && (PowercapDomainSupportsCpc(old_s, domain_id) || cpli == 0)
        && PowercapDomainSupportsCaiConfig(old_s, domain_id)
        && flags == 0
        && cai != 0
        && IsPowercapCaiSupported(old_s, domain_id, cai)
        && IsAgentAllowedToSetPowercapCai(old_s, domain_id)
        && PowercapCai(new_s, domain_id, cpli) == cai
        && PowercapStateUnchangedExceptCai(old_s, new_s, domain_id, cpli)))
}
