pub open spec fn power_state_get__3_3_2_7_spec(status: Int32, power_state: UInt32, domain_id: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidPowerDomain(domain_id) ==> ResultEqual(status, NOT_FOUND))
    && (ResultEqual(status, SUCCESS) ==> power_state == PowerDomainAt(domain_id).power_state)
    && (IsDevicePowerDomain(domain_id) ==> power_state == PowerDomainAt(domain_id).power_state)
    && (old_s == new_s)
}