pub open spec fn powercap_mai_get__3_10_3_9_spec(result: Int32, mai: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidPowerCapDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsPowerCapMaiGetSupported(old_s, domain_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> mai == EnforcedPowerCapMai(old_s, domain_id))
    && (ResultEqual(result, SUCCESS) ==> old_s == new_s)
}