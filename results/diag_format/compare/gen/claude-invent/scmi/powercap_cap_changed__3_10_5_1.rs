pub open spec fn powercap_cap_changed__3_10_5_1_spec(agent_id: u32, domain_id: u32, power_cap: u32, cai: u32, cpli: u32, old_s: S, new_s: S) -> bool {
    (PowercapDomainIsValid(old_s, domain_id) && PowercapAgentRegisteredForCapChangeNotification(old_s, domain_id)
        ==> (power_cap == PowercapDomainCurrentPowerCap(old_s, domain_id)
            && cai == PowercapDomainCurrentCai(old_s, domain_id)
            && (!PowercapDomainSupportsCpc(old_s, domain_id) ==> cpli == 0)
            && PowercapDomainCurrentPowerCap(new_s, domain_id) == PowercapDomainCurrentPowerCap(old_s, domain_id)
            && PowercapDomainCurrentCai(new_s, domain_id) == PowercapDomainCurrentCai(old_s, domain_id)))
}
