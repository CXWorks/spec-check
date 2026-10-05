pub open spec fn powercap_cap_get__3_10_3_7_spec(result: Int32, power_cap: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidPowerCapDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidCpli(old_s, domain_id, cpli) ==> ResultEqual(result, NOT_FOUND))
    && (!IsPowerCapGetSupported(old_s, domain_id, cpli) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> power_cap == CurrentEnforcedPowerCap(old_s, domain_id, cpli))
    && (power_cap == 0 ==> PowerCappingDisabled(old_s, domain_id))
    && (old_s == new_s)
}