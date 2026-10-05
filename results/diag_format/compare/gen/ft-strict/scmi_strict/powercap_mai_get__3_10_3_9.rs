pub open spec fn powercap_mai_get__3_10_3_9_spec(domain_id: UInt32, result: Result<Int32, int>, mai: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidPowerCapDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
  && (!IsPowerCapMaiGetSupported(old_s, domain_id) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> mai == EnforcedPowerCapMai(new_s, domain_id))
  && ((IsValidPowerCapDomain(old_s, domain_id) &&
       IsPowerCapMaiGetSupported(old_s, domain_id))
    ==> result == SUCCESS)
}