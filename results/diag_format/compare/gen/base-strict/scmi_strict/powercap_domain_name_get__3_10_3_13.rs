pub open spec fn powercap_domain_name_get__3_10_3_13_spec(result: Int32, flags: UInt32, name: UInt8[64], old_s: S, new_s: S) -> bool {
    (!PowercapDomainExists(domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (PowercapDomainExists(domain_id(old_s)) ==> (ResultEqual(result, SUCCESS) && flags == 0 && IsNullTerminatedAscii(name, 64) && IsPowercapDomainExtendedName(name, domain_id(old_s))))
    && (old_s == new_s)
}