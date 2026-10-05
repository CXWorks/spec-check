pub open spec fn power_state_get__3_3_2_7_spec(domain_id: u32, status: i32, power_state: u32, old_s: S, new_s: S) -> bool {
    (!IsValidPowerDomain(old_s, domain_id) ==> status == NOT_FOUND)
    && (status == SUCCESS ==> (IsValidPowerDomain(old_s, domain_id) && power_state == PowerDomainState(old_s, domain_id)))
    && (new_s == old_s)
}
