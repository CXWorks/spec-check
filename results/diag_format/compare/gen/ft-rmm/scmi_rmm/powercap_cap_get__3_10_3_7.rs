pub open spec fn powercap_cap_get__3_10_3_7_spec(domain_id: UInt32, cpli: UInt32, status: Int32, power_cap: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidCpli(old_s, domain_id, cpli) ==> ResultEqual(status, NOT_FOUND))
  && (!IsRequestSupported(old_s, domain_id, cpli) ==> ResultEqual(status, NOT_SUPPORTED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> power_cap == EnforcedPowerCap(new_s, domain_id, cpli))
  && (ResultEqual(status, SUCCESS) ==> (power_cap == 0) == PowerCappingDisabled(new_s, domain_id))
  && ((IsValidPowercapDomain(old_s, domain_id) &&
       IsValidCpli(old_s, domain_id, cpli) &&
       IsRequestSupported(old_s, domain_id, cpli))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> EnforcedPowerCap(new_s, domain_id, cpli) == EnforcedPowerCap(old_s, domain_id, cpli))
  && (result != SUCCESS
    ==> PowerCappingDisabled(new_s, domain_id) == PowerCappingDisabled(old_s, domain_id))
}