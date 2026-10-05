pub open spec fn powercap_cap_set_complete__3_10_4_1_spec(status: Int32, domain_id: UInt32, power_cap: UInt32, cpli: UInt32, old_s: S, new_s: S) -> bool {
  (ResultEqual(status, SUCCESS))
  && (PowerCapRequestCompleted(new_s, domain_id, power_cap))
  && (!DomainSupportsCpc(new_s, domain_id) ==> cpli == 0)
  && ((!(ResultEqual(status, SUCCESS)))
    ==> PowerCapRequestCompleted(new_s, domain_id, power_cap))
  && ((!(ResultEqual(status, SUCCESS)))
    ==> (DomainSupportsCpc(new_s, domain_id) ==> cpli != 0))
}