pub open spec fn power_domain_name_get__3_3_2_10_spec(result: Int32, flags: UInt32, ext_name: UInt8[64], old_s: S, new_s: S) -> bool {
    (!PowerDomainExists(domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (PowerDomainExists(domain_id(old_s)) ==> (ResultEqual(result, SUCCESS) && flags == 0 && ext_name == PowerDomainExtendedName(domain_id(old_s)) && IsNullTerminatedAscii(ext_name, 64)))
    && (old_s == new_s)
}