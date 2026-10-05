pub open spec fn power_state_get__3_3_2_7_spec(result: int32, power_state: uint32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> !PowerDomainExists(old_s, domain_id))
    && (result == SUCCESS ==> PowerDomainExists(old_s, domain_id))
    && (result == SUCCESS ==> power_state == PowerDomainState(old_s, domain_id))
}