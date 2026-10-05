pub open spec fn voltage_domain_name_get__3_9_2_11_spec(status: Int32, flags: UInt32, name: UInt8[64], old_s: S, new_s: S) -> bool {
    (!VoltageDomainExists(domain_id(old_s)) ==> ResultEqual(status, NOT_FOUND))
    && (VoltageDomainExists(domain_id(old_s)) ==> (ResultEqual(status, SUCCESS) && flags == 0 && name == VoltageDomainExtendedName(domain_id(old_s)) && IsNullTerminatedAscii(name, 64)))
    && (old_s == new_s)
}