pub open spec fn powercap_cap_get__3_10_3_7_spec(domain_id: UInt32, cpli: UInt32, result: Result<Int32, UInt32>, power_cap: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidPowerCapDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
  && (!IsValidCpli(old_s, domain_id, cpli) ==> ResultEqual(result, NOT_FOUND))
  && (!IsPowerCapGetSupported(old_s, domain_id, cpli) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> power_cap == CurrentEnforcedPowerCap(new_s, domain_id, cpli))
  && (result == SUCCESS ==> power_cap == 0 ==> PowerCappingDisabled(new_s, domain_id))
  && ((IsValidPowerCapDomain(old_s, domain_id) &&
       IsValidCpli(old_s, domain_id, cpli) &&
       IsPowerCapGetSupported(old_s, domain_id, cpli))
    ==> ResultEqual(result, SUCCESS))
  && (result != SUCCESS
    ==> power_cap == CurrentEnforcedPowerCap(new_s, domain_id, cpli))
  && (result != SUCCESS
    ==> PowerCappingDisabled(new_s, domain_id))
}