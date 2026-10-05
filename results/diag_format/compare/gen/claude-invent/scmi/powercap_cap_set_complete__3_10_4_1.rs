pub open spec fn powercap_cap_set_complete__3_10_4_1_spec(status: i32, domain_id: u32, power_cap: u32, cpli: u32, old_s: S, new_s: S) -> bool {
    (!PowercapDomainSupportsCpc(old_s, domain_id) ==> cpli == 0)
    && (status == 0 ==> PowercapDomainPowerCap(new_s, domain_id, cpli) == power_cap)
}
