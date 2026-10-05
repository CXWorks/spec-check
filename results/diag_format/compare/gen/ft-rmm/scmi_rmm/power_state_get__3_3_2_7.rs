pub open spec fn power_state_get__3_3_2_7_spec(domain_id: UInt32, status: Int32, power_state: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidPowerDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> power_state == PowerDomainAt(new_s, domain_id).power_state)
  && (IsDevicePowerDomain(old_s, domain_id) ==> power_state == PowerDomainAt(new_s, domain_id).power_state)
  && ((!(IsValidPowerDomain(old_s, domain_id)))
    ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS)
    ==> ResultEqual(status, SUCCESS))
}