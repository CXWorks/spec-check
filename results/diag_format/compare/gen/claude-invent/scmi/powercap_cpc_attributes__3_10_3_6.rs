pub open spec fn powercap_cpc_attributes__3_10_3_6_spec(domain_id: UInt32, desc_index: UInt32, status: i32, num_cpl: UInt32, desc: Seq<CpliDesc>, old_s: S, new_s: S) -> bool {
    (!PowercapDomainExists(old_s, domain_id) ==> status == NOT_FOUND)
    && ((PowercapDomainExists(old_s, domain_id) && !PowercapDomainSupportsCpc(old_s, domain_id)) ==> status == NOT_SUPPORTED)
    && ((PowercapDomainExists(old_s, domain_id) && PowercapDomainSupportsCpc(old_s, domain_id)
        && (desc_index as int) >= (PowercapCpcNumCpli(old_s, domain_id) as int)) ==> status == OUT_OF_RANGE)
    && (status == SUCCESS ==> (
        PowercapDomainExists(old_s, domain_id)
        && PowercapDomainSupportsCpc(old_s, domain_id)
        && (desc_index as int) < (PowercapCpcNumCpli(old_s, domain_id) as int)
        && ((num_cpl & 0xFFFF) as int) == desc.len()
        && (desc_index as int) + desc.len() <= (PowercapCpcNumCpli(old_s, domain_id) as int)
        && ((num_cpl >> 16) as int) == (PowercapCpcNumCpli(old_s, domain_id) as int) - (desc_index as int) - desc.len()
        && (forall|i: int| 0 <= i < desc.len() ==> desc[i] == PowercapCpcCpliDescAt(old_s, domain_id, (desc_index as int) + i))
        && (forall|i: int| 0 <= i < desc.len() - 1 ==> desc[i].cpli < desc[i + 1].cpli)
        && (forall|i: int| 0 <= i < desc.len() ==> (
            (desc[i].flags >> 1) == 0
            && desc[i].min_power_cap != 0
            && desc[i].max_power_cap != 0
            && (desc[i].min_power_cap != desc[i].max_power_cap ==> desc[i].power_cap_step != 0)
            && (desc[i].min_cai != desc[i].max_cai ==> desc[i].cai_step != 0)
            && desc[i].name.len() == 16
            && (exists|j: int| 0 <= j < 16 && desc[i].name[j] == 0u8)
        ))
        && new_s == old_s
    ))
}
