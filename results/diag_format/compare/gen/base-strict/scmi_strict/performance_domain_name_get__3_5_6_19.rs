pub open spec fn performance_domain_name_get__3_5_6_19_spec(status: Int32, flags: UInt32, name: UInt8[64], old_s: S, new_s: S) -> bool {
    (!PerformanceDomainExists(domain_id(old_s)) ==> ResultEqual(status, NOT_FOUND))
    && (PerformanceDomainExists(domain_id(old_s)) ==> (ResultEqual(status, SUCCESS) && flags == 0 && IsNullTerminatedAsciiString(name, 64) && name == PerformanceDomainExtendedName(domain_id(old_s))))
    && (old_s == new_s)
}