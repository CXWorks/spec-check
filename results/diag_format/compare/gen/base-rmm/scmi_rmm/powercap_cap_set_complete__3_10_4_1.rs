pub open spec fn powercap_cap_set_complete__3_10_4_1_spec(result: RsiCommandReturnCode, domain_id: UInt32, power_cap: UInt32, cpli: UInt32, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==> PowerCapDomain(new_s, domain_id).power_cap == power_cap)
    && (!DomainSupportsCpc(old_s, domain_id) ==> cpli == 0)
    && true
}