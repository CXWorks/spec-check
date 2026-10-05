pub open spec fn performance_domain_name_get__3_5_6_19_spec(result: Int32, flags: UInt32, name: UInt8[64], old_s: S, new_s: S) -> bool {
    (!PerformanceDomainExists(old_s, domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (PerformanceDomainExists(old_s, domain_id(old_s)) ==> (ResultEqual(result, SUCCESS) && flags == 0 && name == PerformanceDomainExtendedName(old_s, domain_id(old_s)) && IsNullTerminatedAscii(name, 64)))
}