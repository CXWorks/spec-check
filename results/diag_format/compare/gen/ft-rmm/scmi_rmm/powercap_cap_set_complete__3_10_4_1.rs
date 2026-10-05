pub open spec fn powercap_cap_set_complete__3_10_4_1_spec(status: Int32, domain_id: UInt32, power_cap: UInt32, cpli: UInt32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Ok() ==> status == 0)
  && (result.is_Ok() ==> PowerCapDomain(new_s, domain_id).power_cap == power_cap)
  && (result.is_Ok() && !DomainSupportsCpc(old_s, domain_id) ==> cpli == 0)
  && ((!(result.is_Ok()))
    ==> PowerCapDomain(new_s, domain_id).power_cap == PowerCapDomain(old_s, domain_id).power_cap)
}