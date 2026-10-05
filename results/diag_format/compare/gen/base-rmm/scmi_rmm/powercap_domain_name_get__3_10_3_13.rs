pub open spec fn powercap_domain_name_get__3_10_3_13_spec(result: Int32, flags: UInt32, name: UInt8[64], old_s: S, new_s: S) -> bool {
    (!PowercapDomainExists(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (ResultEqual(result, SUCCESS) && name == PowercapDomainExtendedName(domain_id) && flags == 0))
}