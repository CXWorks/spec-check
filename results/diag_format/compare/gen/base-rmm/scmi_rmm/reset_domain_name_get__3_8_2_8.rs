pub open spec fn reset_domain_name_get__3_8_2_8_spec(status: Int32, flags: UInt32, name: UInt8[64], old_s: S, new_s: S) -> bool {
    (!ResetDomainExists(domain_id(old_s)) ==> ResultEqual(status, NOT_FOUND))
    && (ResetDomainExists(domain_id(old_s)) ==> (ResultEqual(status, SUCCESS) && flags == 0 && name == ResetDomainExtendedName(domain_id(old_s)) && IsNullTerminatedAscii(name, 64)))
    && (old_s == new_s)
}